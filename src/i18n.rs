use serde_json::Value;
use std::collections::HashMap;
use std::env;

#[allow(dead_code)]
pub struct TranslationManager {
    current_locale: String,
    translations: HashMap<String, String>,
}

impl TranslationManager {
    #[allow(dead_code)]
    pub fn load_detected() -> Self {
        let lang = env::var("LANG")
            .or_else(|_| env::var("LC_ALL"))
            .unwrap_or_else(|_| "en_US.UTF-8".to_string());

        let locale_code = if lang.starts_with("id") { "id" } else { "en" };

        Self::load_locale(locale_code)
    }

    pub fn load_locale(locale: &str) -> Self {
        let json_str = match locale {
            "id" => include_str!("../translations/id.json"),
            _ => include_str!("../translations/en.json"),
        };

        let map: HashMap<String, String> =
            match serde_json::from_str::<HashMap<String, Value>>(json_str) {
                Ok(parsed) => parsed
                    .into_iter()
                    .filter_map(|(k, v)| v.as_str().map(|s| (k, s.to_string())))
                    .collect(),
                Err(_) => HashMap::new(),
            };

        Self {
            current_locale: locale.to_string(),
            translations: map,
        }
    }

    #[allow(dead_code)]
    pub fn tr<'a>(&'a self, key: &'a str) -> &'a str {
        self.translations
            .get(key)
            .map(|s| s.as_str())
            .unwrap_or(key)
    }

    #[allow(dead_code)]
    pub fn locale(&self) -> &str {
        &self.current_locale
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_i18n_fallback() {
        let manager = TranslationManager::load_locale("en");
        assert_eq!(manager.tr("cavalry.launch"), "Launch");
        assert_eq!(manager.tr("unknown_key"), "unknown_key");
    }

    #[test]
    fn test_i18n_indonesian() {
        let manager = TranslationManager::load_locale("id");
        assert_eq!(manager.tr("cavalry.launch"), "Jalankan");
    }
}
