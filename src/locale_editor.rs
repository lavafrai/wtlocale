use std::error::Error;
use crate::game_controller::{
    create_game_controller, get_config_blk_path, is_debug_localization_enabled,
    is_localization_files_created,
};
use arboard::Clipboard;
use std::path::Path;
use std::rc::Rc;
use std::sync::Arc;
use std::thread;
use slint::{ModelRc, SharedString, VecModel, invoke_from_event_loop};
use crate::locale_cotroller::LocaleController;

slint::include_modules!();

pub fn run(game_folder: &Path) -> Result<(), Box<dyn Error>> {
    // Initialize game controller
    let controller = Arc::new(create_game_controller(game_folder));
    let locale_controller = Arc::new(LocaleController::new(&controller));
    let initial_debug_enabled = is_debug_localization_enabled(&controller);
    let initial_localization_files_created = is_localization_files_created(&controller);
    // Initialize UI
    let ui = LocaleEditorUI::new()?;
    let ui_weak = ui.as_weak();

    if initial_debug_enabled && initial_localization_files_created {
        update_locale_state(&ui, Arc::clone(&locale_controller), None);
    }


    // Copy debug locale to clipboard
    ui.on_request_clipboard_locale_debug(|| {
        let mut clipboard = Clipboard::new().expect("Failed to init clipboard");
        clipboard
            .set_text("testLocalization:b=yes".to_owned())
            .expect("Failed to set clipboard text");
    });

    // Open config.blk
    {
        let controller = Arc::clone(&controller);
        ui.on_request_open_config_blk(move || {
            let config_path = get_config_blk_path(&controller);
            opener::open(config_path).expect("Failed to open config file");
        });
    }

    // Recheck debug localization state
    {
        let controller = Arc::clone(&controller);
        let locale_controller = Arc::clone(&locale_controller);
        let ui_handle = ui_weak.clone();
        ui.on_request_localization_state_recheck(move || {
            let enabled = is_debug_localization_enabled(&controller);
            if let Some(handle) = ui_handle.upgrade() {
                handle.set_localization_debug_enabled(enabled);
            }

            let files_created = is_localization_files_created(&controller);
            if let Some(handle) = ui_handle.upgrade() {
                handle.set_localization_files_created(files_created);
            }

            if enabled && files_created {
                update_locale_state(&ui_handle.unwrap(), Arc::clone(&locale_controller), None);
            }
        });
    }

    {
        let locale_controller = Arc::clone(&locale_controller);
        let ui_handle = ui_weak.clone();
        ui.on_locale_set(move |new_locale| {
            update_locale_state(&ui_handle.unwrap(), Arc::clone(&locale_controller), Some(new_locale.to_string()));
        });
    }

    // Apply initial state
    ui.set_localization_debug_enabled(initial_debug_enabled);
    ui.set_localization_files_created(initial_localization_files_created);

    ui.run()?;

    Ok(())
}

fn update_locale_state(ui: &LocaleEditorUI, locale_controller: Arc<LocaleController>, selected_locale: Option<String>) {
    // Start asynchronous loading
    ui.set_loading_locales(true);
    let ui_weak = ui.as_weak();
    let controller = Arc::clone(&locale_controller);
    let selected_opt = selected_locale.clone();
    thread::spawn(move || {
        // Load data in background
        let available_locales = controller.get_available_locales();
        let selected = selected_opt
            .or_else(|| available_locales.first().cloned())
            .unwrap_or_default();
        let texts = controller.get_locale_texts(&selected);
        println!("Locale '{}' has {} texts", selected, texts.len());
        // Update UI in UI thread
        invoke_from_event_loop(move || {
            if let Some(handle) = ui_weak.upgrade() {
                let shared: Vec<SharedString> = available_locales
                    .iter()
                    .map(|l| SharedString::from(l.clone()))
                    .collect();
                let model = Rc::new(VecModel::from(shared));
                let model_rc = ModelRc::from(model.clone());
                handle.set_available_locales(model_rc);
                handle.set_selected_locale(SharedString::from(selected.clone()));
                handle.set_loading_locales(false);
            }
        }).expect("");
    });
}
