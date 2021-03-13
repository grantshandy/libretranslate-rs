//! # libretranslate-rs
//! A LibreTranslate API for Rust.
//! ```
//! libretranslate = "0.1.2"
//! ```
//!
//! libretranslate-rs allows you to use open source machine translation in your projects through an easy to use API that connects to the official [webpage](https://libretranslate.com/).
//!
//! ## Example
//! Using it is fairly simple:
//! ```rust
//! use libretranslate::{Translator, Language};
//!
//! fn main() {
//!     let source = Language::Portuguese;
//!     let target = Language::English;
//!     let input = "Olá Mundo!";
//!     let output = Translator::translate(source, target, input).unwrap().output;
//!
//!     println!("Input {}: {}", source.pretty(), input);
//!     println!("Output {}: {}", target.pretty(), output);
//! }
//! ```
//!
//! Output:
//! ```
//! Input Portuguese: Olá Mundo!
//! Output English: Hello world!
//! ```
//!
//! ## Available Languages
//! - English
//! - Arabic
//! - Chinese
//! - French
//! - German
//! - Italian
//! - Japanese
//! - Portuguese
//! - Russian
//! - Spanish
//!
//! Written in Rust, with love by Grant Handy.

use serde_json::Value;
use std::str::FromStr;

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
    pub fn code(&self) -> &'static str {
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
    pub fn pretty(&self) -> &'static str {
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
    DetectionError,
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
            TranslateError::DetectionError => {
                write!(f, "Error Detecting Language")
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

impl Translator {
    /// Translate text between two languages.
    pub fn translate(source: Language, target: Language, input: &str) -> Result<Translator, TranslateError> {
        match ureq::post("https://libretranslate.com/translate").send_json(ureq::json!({
            "q": input,
            "source": source.code(),
            "target": target.code(),
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
}
