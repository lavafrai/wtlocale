use crate::game_controller::{GameController, get_locale_path};

pub struct LocaleText {
    lang: String,
    text: String,
    category: String,
    max_chars: u32,
}

pub struct LocaleController {
    locale_path: String,
}

impl LocaleController {
    pub fn new(game_controller: &GameController) -> Self {
        let locale_path = get_locale_path(game_controller);
        LocaleController {
            locale_path: locale_path
                .to_str()
                .expect("Error converting path to string")
                .to_owned(),
        }
    }

    pub fn get_available_locales(&self) -> Vec<String> {
        println!("Reading locale file: {}", self.locale_path);
        let menu_csv_path = self.locale_path.to_owned() + "/menu.csv";
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(true)
            .delimiter(b';')
            .from_path(menu_csv_path)
            .expect("Failed to open locale file");

        let headers = reader.headers().expect("Failed to read headers");
        let mut locales = Vec::new();
        for header in headers.iter() {
            if header != "<ID|readonly|noverify>" && header != "<Comments>" && header != "<max_chars>" {
                let pretty_locale = header.replace("<", "").replace(">", "");
                locales.push(pretty_locale.to_string());
            }
        }
        locales
    }
}
