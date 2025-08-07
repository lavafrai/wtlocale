use std::cell::RefCell;
use std::rc::Rc;

slint::include_modules!();


pub fn get() -> Result<String, Box<dyn std::error::Error>> {
    let selected_folder = Rc::new(RefCell::new(None));

    let ui = GameFolderSelector::new()?;
    let ui_handle = ui.as_weak();

    let selected_folder_clone = Rc::clone(&selected_folder);
    ui.on_request_file_dialog_open(move || {
        *selected_folder_clone.borrow_mut() = open_game_folder_dialog();
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