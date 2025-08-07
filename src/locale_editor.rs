use std::collections::HashMap;
use std::path::Path;
use regex::Regex;
use rustring_builder::StringBuilder;

pub fn run(game_folder: &Path) -> Result<(), Box<dyn std::error::Error>> {
    read_debug_config(game_folder)?;
    Ok(())
}

fn read_debug_config(game_folder: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let config_path = game_folder.join("config.blk");
    if !config_path.exists() {
        return Err("Debug config file not found".into());
    }

    let content = std::fs::read_to_string(config_path)?;
    let mut result: HashMap<String, String> = HashMap::new();
    let mut debug_builder = StringBuilder::new();

    let debug_regex = Regex::new(r"debug\{([\s\S]+?)}").unwrap();
    for cap in debug_regex.captures_iter(&content) {
        if let Some(debug_content) = cap.get(1) {
            let debug_str = debug_content.as_str();
            debug_builder.append(debug_content.as_str());
        }
    }
    let debug_string = debug_builder.to_string();

    println!("Debug string\n---");
    println!("{}", debug_string);
    println!("---");

    let parameter_regex = Regex::new(r"(\S+?)=(\S+?)\r?\n").unwrap();
    for cap in parameter_regex.captures_iter(debug_string.as_str()) {
        if let (Some(key), Some(value)) = (cap.get(1), cap.get(2)) {
            result.insert(key.as_str().to_string(), value.as_str().to_string());
        }
    }
    println!("Parsed Debug Config: {:?}", result);

    Ok(())
}

