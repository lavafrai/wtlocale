#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod game_folder;
mod locale_editor;
mod game_controller;
mod locale_cotroller;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let game_folder = game_folder::get()?;
    println!("Game folder selected: {}", game_folder);

    let game_path = std::path::PathBuf::from(&game_folder);
    locale_editor::run(&game_path)?;

    Ok(())
}
