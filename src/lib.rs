

use serde_json::Value;
use std::str::FromStr;
use whatlang::Lang;

/// Languages that can used for input and output of the ['translate'] function.
#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Language {
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
	pub fn from(s: &str) -> Result<Self, LanguageError> {
		return Self::from_str(s);
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
            "french"=> Ok(Language::French),
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

/// Errors that could be outputed by a Language.
#[derive(Debug, Clone, PartialEq)]
pub enum LanguageError {
    FormatError(String),
}

impl std::error::Error for LanguageError {}

impl std::fmt::Display for LanguageError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            LanguageError::FormatError(error) => {
                write!(f, "Unknown Language: {}", error.to_string())
            }
        }
    }
}

/// Errors that could be outputed by the translator.
#[derive(Debug, Clone)]
pub enum TranslateError {
    HttpError(String),
    ParseError(String),
    DetectError,
}

impl std::error::Error for TranslateError {}

impl std::fmt::Display for TranslateError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TranslateError::HttpError(error) => {
                write!(f, "HTTP Request Error: {}", error.to_string())
            }
            TranslateError::ParseError(error) => {
                write!(f, "JSON Parsing Error: {}", error.to_string())
            }
            TranslateError::DetectError => {
                write!(f, "Language Detection Error")
            }
        }
    }
}

pub struct Translator {
    pub source: Language,
    pub target: Language,
    pub input: String,
    pub output: String,
}

/// Translate text between two languages.
pub fn translate(source: Option<Language>, target: Language, input: &str) -> Result<Translator, TranslateError> {
    let source= match source {
        Some(data) => data,
        None => {
            let info = match whatlang::detect(input) {
                Some(data) => data,
                None => return Err(TranslateError::DetectError),
            };

            match info.lang() {
                Lang::Eng => Language::English,
                Lang::Ara => Language::Arabic,
                Lang::Fra => Language::French,
                Lang::Deu => Language::German,
                Lang::Ita => Language::Italian,
                Lang::Por => Language::Portuguese,
                Lang::Rus => Language::Russian,
                Lang::Spa => Language::Spanish,
                Lang::Jpn => Language::Japanese,
                _ => return Err(TranslateError::DetectError),
            }
        },
    };

    match ureq::post("https://libretranslate.com/translate").send_json(ureq::json!({
        "q": input,
        "source": source.as_code(),
        "target": target.as_code(),
    })) {
        Ok(data) => {
            let string: String = match data.into_string() {
                Ok(data) => data,
                Err(error) => {
                    return Err(TranslateError::ParseError(error.to_string()));
                }
            };

            let parsed_json: Value = match serde_json::from_str(&string) {
                Ok(parsed_json) => parsed_json,
                Err(error) => {
                    return Err(TranslateError::ParseError(error.to_string()));
                }
            };

            let output = match &parsed_json["translatedText"] {
                Value::String(output) => output,
                _ => {
                    return Err(TranslateError::ParseError(String::from(
                        "Unable to find translatedText in parsed JSON",
                    )))
                }
            };

            let input = input.to_string();
            let output = output.to_string();

            return Ok(Translator {
                source,
                target,
                input,
                output,
            });
        },
        Err(error) => return Err(TranslateError::HttpError(error.to_string())),
    };
}


pub trait Translate {
    fn to_english(&self) -> Result<String, TranslateError>;
    fn to_arabic(&self) -> Result<String, TranslateError>;
    fn to_french(&self) -> Result<String, TranslateError>;
    fn to_german(&self) -> Result<String, TranslateError>;
    fn to_italian(&self) -> Result<String, TranslateError>;
    fn to_japanese(&self) -> Result<String, TranslateError>;
    fn to_portuguese(&self) -> Result<String, TranslateError>;
    fn to_russian(&self) -> Result<String, TranslateError>;
    fn to_spanish(&self) -> Result<String, TranslateError>;
}

impl<T> Translate for T
    where T: AsRef<str>
{    
    fn to_english(&self) -> Result<String, TranslateError> {
        match translate(None, Language::English, self.as_ref()) {
            Ok(data) => Ok(data.output),
            Err(error) => return Err(error),
        }
    }
    fn to_arabic(&self) -> Result<String, TranslateError> {
        match translate(None, Language::Arabic, self.as_ref()) {
            Ok(data) => Ok(data.output),
            Err(error) => return Err(error),
        }
    }
    fn to_french(&self) -> Result<String, TranslateError> {
        match translate(None, Language::French, self.as_ref()) {
            Ok(data) => Ok(data.output),
            Err(error) => return Err(error),
        }
    }
    fn to_german(&self) -> Result<String, TranslateError> {
        match translate(None, Language::German, self.as_ref()) {
            Ok(data) => Ok(data.output),
            Err(error) => return Err(error),
        }
    }
    fn to_italian(&self) -> Result<String, TranslateError> {
        match translate(None, Language::Italian, self.as_ref()) {
            Ok(data) => Ok(data.output),
            Err(error) => return Err(error),
        }
    }
    fn to_japanese(&self) -> Result<String, TranslateError> {
        match translate(None, Language::Japanese, self.as_ref()) {
            Ok(data) => Ok(data.output),
            Err(error) => return Err(error),
        }
    }
    fn to_portuguese(&self) -> Result<String, TranslateError> {
        match translate(None, Language::Portuguese, self.as_ref()) {
            Ok(data) => Ok(data.output),
            Err(error) => return Err(error),
        }
    }
    fn to_russian(&self) -> Result<String, TranslateError> {
        match translate(None, Language::Russian, self.as_ref()) {
            Ok(data) => Ok(data.output),
            Err(error) => return Err(error),
        }
    }
    fn to_spanish(&self) -> Result<String, TranslateError> {
        match translate(None, Language::Spanish, self.as_ref()) {
            Ok(data) => Ok(data.output),
            Err(error) => return Err(error),
        }
    }
}