use std::error::Error;
use crate::game_controller::{
    create_game_controller, get_config_blk_path, is_debug_localization_enabled,
    is_localization_files_created,
};
use arboard::Clipboard;
use std::path::Path;
use std::rc::Rc;
use slint::{ModelRc, SharedString, VecModel};
use crate::locale_cotroller::LocaleController;

slint::include_modules!();

pub fn run(game_folder: &Path) -> Result<(), Box<dyn Error>> {
    // Initialize game controller
    let controller = Rc::new(create_game_controller(game_folder));
    let locale_controller = Rc::new(LocaleController::new(&controller));
    let initial_debug_enabled = is_debug_localization_enabled(&controller);
    let initial_localization_files_created = is_localization_files_created(&controller);
    // Initialize UI
    let ui = LocaleEditorUI::new()?;
    let ui_weak = ui.as_weak();

    if initial_debug_enabled && initial_localization_files_created {
        update_locale_state(&ui, &locale_controller, None);
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
        let controller = Rc::clone(&controller);
        ui.on_request_open_config_blk(move || {
            let config_path = get_config_blk_path(&controller);
            opener::open(config_path).expect("Failed to open config file");
        });
    }

    // Recheck debug localization state
    {
        let controller = Rc::clone(&controller);
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
                update_locale_state(&ui_handle.unwrap(), &locale_controller, None);
            }
        });
    }

    // Apply initial state
    ui.set_localization_debug_enabled(initial_debug_enabled);
    ui.set_localization_files_created(initial_localization_files_created);

    ui.run()?;

    Ok(())
}

fn update_locale_state(ui: &LocaleEditorUI, locale_controller: &Rc<LocaleController>, selected_locale: Option<String>) {
    let available_locales = locale_controller.get_available_locales();
    let shared_locales: Vec<SharedString> = available_locales
        .iter()
        .map(|locale| SharedString::from(locale.clone()))
        .collect();
    let model = Rc::new(VecModel::from(shared_locales));
    let model_rc = ModelRc::from(model.clone());
    ui.set_available_locales(model_rc);
}
