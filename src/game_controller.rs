use std::collections::HashMap;
use std::error::Error;
use std::path::{Path, PathBuf};
use regex::Regex;
use rustring_builder::StringBuilder;

#[derive(Debug, Clone)]
pub struct GameController {
    game_path: PathBuf,
}

pub fn create_game_controller(
    game_path: &Path
) -> GameController {
    GameController {
        game_path: game_path.to_path_buf(),
    }
}

pub fn is_debug_localization_enabled(
    game_controller: &GameController
) -> bool {
    match read_debug_config(&game_controller.game_path) {
        Ok(params) => params.get("testLocalization:b").map_or(false, |v| v == "yes"),
        Err(_) => false,
    }
}

pub fn is_localization_files_created(
    game_controller: &GameController
) -> bool {
    let localization_path = game_controller.game_path.join("lang");
    localization_path.exists() && localization_path.is_dir() && localization_path.join("menu.csv").exists()
}

pub fn get_locale_path(
    game_controller: &GameController,
) -> PathBuf {
    game_controller.game_path.join("lang")
}

fn read_debug_config(
    game_path: &Path
) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let config_path = game_path.join("config.blk");
    if !config_path.exists() {
        return Err("Debug config file not found".into());
    }

    let content = std::fs::read_to_string(config_path)?;
    let mut result: HashMap<String, String> = HashMap::new();
    let mut debug_builder = StringBuilder::new();

    let debug_regex = Regex::new(r"debug\{([\s\S]+?)}").unwrap();
    for cap in debug_regex.captures_iter(&content) {
        if let Some(debug_content) = cap.get(1) {
            debug_builder.append(debug_content.as_str());
        }
    }
    let debug_string = debug_builder.to_string();

    let parameter_regex = Regex::new(r"(\S+?)=(\S+?)\r?\n").unwrap();
    for cap in parameter_regex.captures_iter(debug_string.as_str()) {
        if let (Some(key), Some(value)) = (cap.get(1), cap.get(2)) {
            result.insert(key.as_str().to_string(), value.as_str().to_string());
        }
    }
    // debug config parsed: result

    Ok(result)
}

pub fn get_config_blk_path(game_controller: &GameController) -> PathBuf {
    game_controller.game_path.join("config.blk")
}