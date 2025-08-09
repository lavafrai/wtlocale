use crate::game_controller::{ GameController, get_locale_path };


#[derive(Debug)]
#[derive(Clone)]
pub struct LocaleText {
    pub lang: String,
    pub tag: String,
    pub text: String,
    pub category: String,
    pub max_chars: u32,
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

    pub fn get_locale_categories(&self) -> Vec<String> {
        let denied_categories = vec!["_missing"];
        let files = std::fs::read_dir(&self.locale_path)
            .expect("Failed to read locale directory");
        let mut categories = Vec::new();
        for file in files {
            if let Ok(file) = file {
                let path = file.path();
                if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                    let category_name = path.file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or("");
                    if ext == "csv" && !denied_categories.contains(&category_name) {
                        categories.push(category_name.to_string());
                    }
                }
            }
        }

        categories
    }

    fn get_locale_column_index(headers: &csv::StringRecord, locale: &String) -> usize {
        let header_name = format!("<{}>", locale);
        headers.iter()
            .position(|h| h == header_name)
            .expect(&format!("Locale header '{}' not found", header_name))
    }

    pub fn get_locale_text_for_category(&self, locale: &String, category: &String) -> Vec<LocaleText> {
        let file_path = format!("{}/{}.csv", self.locale_path, category);
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(true)
            .delimiter(b';')
            .from_path(&file_path)
            .expect(&format!("Failed to open CSV for category {}", category));

        let headers = reader.headers().expect("Failed to read headers");

        let tag_idx = headers.iter()
            .position(|h| h == "<ID|readonly|noverify>")
            .expect("<ID|readonly|noverify> header not found");
        let locale_idx = Self::get_locale_column_index(headers, locale);
        let max_chars_idx = headers.iter()
            .position(|h| h == "<max_chars>")
            .expect("<max_chars> header not found");

        reader.records()
            .map(|res| {
                let record = res.expect("Failed to read record");
                let tag = record.get(tag_idx).unwrap_or("").to_string();
                let text = record.get(locale_idx).unwrap_or("").to_string();
                let max_chars = record.get(max_chars_idx)
                    .and_then(|s| s.parse::<u32>().ok())
                    .unwrap_or(0);
                LocaleText {
                    lang: locale.clone(),
                    tag,
                    text,
                    category: category.clone(),
                    max_chars,
                }
            })
            .collect()
    }

    pub fn get_locale_texts(&self, locale: &String) -> Vec<LocaleText> {
        self.get_locale_categories().iter()
            .flat_map(|cat| self.get_locale_text_for_category(locale, cat))
            .collect()
    }
}
