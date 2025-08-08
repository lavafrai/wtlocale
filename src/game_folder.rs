use std::cell::RefCell;
use std::rc::Rc;

slint::include_modules!();


pub fn get() -> Result<String, Box<dyn std::error::Error>> {
    let selected_folder = Rc::new(RefCell::new(None));

    let ui = GameFolderSelectorUI::new()?;
    let ui_handle_1 = ui.as_weak();
    let ui_handle_2 = ui.as_weak();

    let selected_folder_clone = Rc::clone(&selected_folder);
    ui.on_request_file_dialog_open(move || {
        let result = open_game_folder_dialog();

        if let Some(path) = result {
            let ui = ui_handle_1.unwrap();
            *selected_folder_clone.borrow_mut() = Some(path.clone());
            ui.set_selected_folder_text(path.clone().into());
            if check_folder_contains_game(&path) {
                ui.set_error("".into());
                ui.set_continuation_allowed(true);
            } else {
                ui.set_error("Не удалось найти игру в выбранном каталоге, убедитесь, что путь указан верно.".into());
                ui.set_continuation_allowed(false);
            }
        }
    });
    ui.on_request_continue(move || {
        let ui = ui_handle_2.unwrap();
        ui.hide().unwrap();
    });

    ui.run().unwrap();

    let final_folder = selected_folder.borrow();
    if final_folder.is_none() {
        Err("No folder selected".into())
    } else {
        Ok(final_folder.as_ref().unwrap().clone())
    }
}

fn open_game_folder_dialog() -> Option<String> {
    rfd::FileDialog::new()
        .set_title("Select Game Directory")
        .pick_folder()
        .map(|path| path.to_string_lossy().to_string())
}

fn check_folder_contains_game(folder: &str) -> bool {
    // check exists
    let path = std::path::Path::new(folder);
    if !path.exists() { return false; }

    // check if it contains required files and subfolders
    let required_files = [
        "launcher.exe",
        "char.vromfs.bin",
        "gaijin_downloader.exe",
        "config.blk"
    ];
    let required_subfolders = [
        "content",
        "win32",
    ];

    for file in required_files {
        if !path.join(file).exists() {
            return false;
        }
    }
    for subfolder in required_subfolders {
        if !path.join(subfolder).is_dir() {
            return false;
        }
    }

    true
}
