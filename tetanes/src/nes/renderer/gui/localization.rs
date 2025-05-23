use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::RwLock;
use tracing::{error, warn};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Language {
    English,
    Chinese,
}

impl Default for Language {
    fn default() -> Self {
        Self::Chinese
    }
}

#[derive(Debug)]
pub struct LocalizedTexts {
    texts: HashMap<String, String>,
}

impl Default for LocalizedTexts {
    fn default() -> Self {
        Self::new()
    }
}

impl LocalizedTexts {
    pub fn new() -> Self {
        Self {
            texts: HashMap::new(),
        }
    }

    pub fn get_text(&self, key: &str) -> Option<String> {
        self.texts.get(key).cloned()
    }

    pub fn insert(&mut self, key: &str, text: String) {
        self.texts.insert(key.to_string(), text);
    }

    pub fn clear(&mut self) {
        self.texts.clear();
    }
}
lazy_static::lazy_static! {
    pub static ref LOCALIZATIONTEXTS: RwLock<LocalizedTexts> = RwLock::new(LocalizedTexts::new());
}

pub struct Localization {
    translations: HashMap<Language, Value>,
    current_language: Language,
}

impl Default for Localization {
    fn default() -> Self {
        Self::new()
    }
}

impl Localization {
    pub fn new() -> Self {
        let mut translations = HashMap::new();
        // 加载英文翻译
        const EN_TRANSLATIONS: &[u8] = include_bytes!("../../../../assets/locales/en.json");
        match serde_json::from_slice(EN_TRANSLATIONS) {
            Ok(json) => {
                translations.insert(Language::English, json);
            }
            Err(e) => error!("Failed to parse English translations: {}", e),
        }

        // 加载中文翻译
        const ZH_TRANSLATIONS: &[u8] = include_bytes!("../../../../assets/locales/zh.json");
        match serde_json::from_slice(ZH_TRANSLATIONS) {
            Ok(json) => {
                translations.insert(Language::Chinese, json);
            }
            Err(e) => error!("Failed to parse Chinese translations: {}", e),
        }

        Self {
            translations,
            current_language: Language::default(),
        }
    }

    pub const fn current_language(&self) -> Language {
        self.current_language
    }

    pub fn set_language(&mut self, language: Language) {
        self.current_language = language;
        if let Ok(mut localized_texts) = LOCALIZATIONTEXTS.try_write() {
            localized_texts.clear();
        }
    }

    pub fn get_text(&self, path: &str) -> String {
        let path = path.trim_start_matches('/');

        // 快速路径：尝试从缓存中读取
        if let Ok(localized_texts) = LOCALIZATIONTEXTS.try_read() {
            if let Some(text) = localized_texts.get_text(path) {
                return text;
            }
        }

        // 如果缓存中没有，则从翻译文件中读取
        let parts: Vec<&str> = path.split('/').collect();

        let translation = match self.translations.get(&self.current_language) {
            Some(t) => t,
            None => {
                warn!(
                    "No translation found for language: {:?}",
                    self.current_language
                );
                return path.to_string();
            }
        };

        let mut current = translation;
        for part in parts {
            match current.get(part) {
                Some(next) => current = next,
                None => {
                    warn!("No translation found for path: {}", path);
                    return path.to_string();
                }
            }
        }

        let result = current.as_str().map(String::from).unwrap_or_else(|| {
            warn!("Invalid translation value for path: {}", path);
            path.to_string()
        });

        // 尝试更新缓存，但不阻塞
        if let Ok(mut localized_texts) = LOCALIZATIONTEXTS.try_write() {
            localized_texts.insert(path, result.clone());
        }

        result
    }
}

lazy_static::lazy_static! {
    pub static ref LOCALIZATION: RwLock<Localization> = RwLock::new(Localization::new());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_translations() {
        let mut localization = Localization::new();

        // 测试英文翻译
        localization.set_language(Language::English);
        assert_eq!(localization.get_text("/ui/quit"), "Quit");
        assert_eq!(localization.get_text("/menu/about_text"), "About");

        // 测试中文翻译
        localization.set_language(Language::Chinese);
        assert_eq!(localization.get_text("/ui/quit"), "退出");
        assert_eq!(localization.get_text("/menu/about_text"), "关于");

        // 测试不存在的路径
        assert_eq!(
            localization.get_text("/nonexistent/path"),
            "/nonexistent/path"
        );
    }
}
