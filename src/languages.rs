use std::str::FromStr;
use crate::error::LanguageError;
use unic_langid::LanguageIdentifier;

/// Languages that can used for input and output of the [`translate`](crate::translate) function.
#[derive(Debug, Clone, PartialEq, Copy, Hash)]
pub enum Language {
    Detect,
    English,
    Arabic,
    Chinese,
    French,
    German,
    Italian,
    Japanese,
    Portuguese,
    Russian,
    Spanish,
}

impl Language {
    /// Return the language with the language code name. (ex. "ar", "de")
    pub fn as_code(&self) -> &'static str {
        match self {
            Language::Detect => "auto",
            Language::English => "en",
            Language::Arabic => "ar",
            Language::Chinese => "zh",
            Language::French => "fr",
            Language::German => "de",
            Language::Italian => "it",
            Language::Japanese => "ja",
            Language::Portuguese => "pt",
            Language::Russian => "ru",
            Language::Spanish => "es",
        }
    }

    /// Return the Language with the full English name. (ex. "Arabic", "German")
    pub fn as_pretty(&self) -> &'static str {
        match self {
            Language::Detect => "Detected",
            Language::English => "English",
            Language::Arabic => "Arabic",
            Language::Chinese => "Chinese",
            Language::French => "French",
            Language::German => "German",
            Language::Italian => "Italian",
            Language::Japanese => "Japanese",
            Language::Portuguese => "Portuguese",
            Language::Russian => "Russian",
            Language::Spanish => "Spanish",
        }
    }

    /// Create a Language from &str like "en" or "French". Case Doesn't matter.
    pub fn from<T: AsRef<str>>(s: T) -> Result<Self, LanguageError> {
        return Self::from_str(s.as_ref());
    }

    /// Create a Language from a unic_langid::LanguageIdentifier.
    pub fn from_unic_langid(s: LanguageIdentifier) -> Result<Self, LanguageError> {
        match s.language.as_str() {
            "en" => Ok(Language::English),
            "ar" => Ok(Language::Arabic),
            "zh" => Ok(Language::Chinese),
            "fr" => Ok(Language::French),
            "de" => Ok(Language::German),
            "it" => Ok(Language::Italian),
            "pt" => Ok(Language::Portuguese),
            "ru" => Ok(Language::Russian),
            "es" => Ok(Language::Spanish),
            "ja" => Ok(Language::Japanese),
            &_ => Err(LanguageError::FormatError("Unknown Language".to_string())),
        }
    }
}

// TODO: Get locale from user to set Language::default().
impl Default for Language {
    fn default() -> Self {
        Language::English
    }
}

impl FromStr for Language {
    type Err = LanguageError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_string().to_lowercase().as_str() {
            "en" => Ok(Language::English),
            "ar" => Ok(Language::Arabic),
            "zh" => Ok(Language::Chinese),
            "fr" => Ok(Language::French),
            "de" => Ok(Language::German),
            "it" => Ok(Language::Italian),
            "pt" => Ok(Language::Portuguese),
            "ru" => Ok(Language::Russian),
            "es" => Ok(Language::Spanish),
            "ja" => Ok(Language::Japanese),
            "english" => Ok(Language::English),
            "arabic" => Ok(Language::Arabic),
            "chinese" => Ok(Language::Chinese),
            "french" => Ok(Language::French),
            "german" => Ok(Language::German),
            "italian" => Ok(Language::Italian),
            "portuguese" => Ok(Language::Portuguese),
            "russian" => Ok(Language::Russian),
            "spanish" => Ok(Language::Spanish),
            "japanese" => Ok(Language::Japanese),
            &_ => Err(LanguageError::FormatError(s.to_string())),
        }
    }
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Language::Detect => write!(f, "auto"),
            Language::English => write!(f, "en"),
            Language::Arabic => write!(f, "ar"),
            Language::Chinese => write!(f, "zh"),
            Language::French => write!(f, "fr"),
            Language::German => write!(f, "de"),
            Language::Italian => write!(f, "it"),
            Language::Portuguese => write!(f, "pt"),
            Language::Russian => write!(f, "ru"),
            Language::Spanish => write!(f, "es"),
            Language::Japanese => write!(f, "ja"),
        }
    }
}