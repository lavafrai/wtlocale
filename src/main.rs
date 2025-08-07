mod game_folder;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let game_folder = game_folder::get()?;
    println!("Game folder selected: {}", game_folder);

    /*ui.on_request_increase_counter(move || {
        let ui = ui_handle.unwrap();
        ui.set_counter(ui.get_counter() + 1);
    });

    let files = FileDialog::new()
        .pick_folder();
    if files.is_some() {
        println!("{:#?}", files);
    }

    ui.run()*/
    Ok(())
}
