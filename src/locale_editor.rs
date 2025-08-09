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
use crate::locale_cotroller::{LocaleController, LocaleText};

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

    {
        let locale_controller = Arc::clone(&locale_controller);
        let ui_handle = ui_weak.clone();
        ui.on_category_set(move |category| {
            let ui = ui_handle.unwrap();
            ui.set_selected_category(SharedString::from(category.to_string()));
            update_locale_state(&ui, Arc::clone(&locale_controller), Some(ui.get_selected_locale().to_string()));
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
    let selected_category = ui.get_selected_category();

    thread::spawn(move || {
        // Load data in background
        let available_locales = controller.get_available_locales();
        let selected = selected_opt
            .or_else(|| available_locales.first().cloned())
            .unwrap_or_default();
        let texts = controller.get_locale_texts(&selected);

        let filtered_texts: Vec<LocaleText> = texts.clone().into_iter()
            .filter(
                |it| {
                    let selected_category = selected_category.to_string();
                    it.category == selected_category
                }
            )
            .collect();

        let categories = controller.get_locale_categories();
        println!("Locale '{}' has {} texts", selected, texts.len());
        // Update UI in UI thread
        invoke_from_event_loop(move || {
            let ui = ui_weak.unwrap();
            if let Some(handle) = ui_weak.upgrade() {
                let shared_locales: ModelRc<SharedString> = string_vec_to_model(available_locales.clone());
                handle.set_available_locales(shared_locales);

                handle.set_selected_locale(SharedString::from(selected.clone()));
                handle.set_loading_locales(false);

                let shared_categories: ModelRc<SharedString> = string_vec_to_model(categories.clone());
                handle.set_available_categories(shared_categories);

                let locale_all_texts = texts.iter()
                    .map(|it| locale_text_to_model(it))
                    .collect::<Vec<_>>();
                let locale_all_model = ModelRc::from(Rc::new(VecModel::from(locale_all_texts)));
                handle.set_locale_texts(locale_all_model);

                let locale_text_models = filtered_texts.iter()
                    .map(|it| locale_text_to_model(it))
                    .collect::<Vec<_>>();
                let locale_model = ModelRc::from(Rc::new(VecModel::from(locale_text_models)));
                ui.set_locale_texts(locale_model);
            }
        }).expect("");
    });
}

fn string_vec_to_model(strings: Vec<String>) -> ModelRc<SharedString> {
    let shared_strings: Vec<SharedString> = strings.into_iter()
        .map(|s| SharedString::from(s))
        .collect();
    ModelRc::from(Rc::new(VecModel::from(shared_strings)))
}

fn locale_text_to_model(text: &LocaleText) -> LocaleTextModel {
    LocaleTextModel {
        tag: SharedString::from(text.tag.clone()),
        text: SharedString::from(text.text.clone()),
        category: SharedString::from(text.category.clone()),
        max_chars: text.max_chars.cast_signed(),
    }
}