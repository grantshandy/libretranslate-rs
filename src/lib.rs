//! A LibreTranslate API for Rust.
//! ```
//! libretranslate = "0.1.2"
//! ```
//!
//! libretranslate-rs allows you to use open source machine translation in your projects through an easy to use API that connects to the official webpage.
//!
//! Using it is fairly simple:
//!
//! ```rust
//! use libretranslate::{translate, Language};
//!
//! fn main() {
//!     let input = "Je te déteste!";
//!     let language_input = Language::French;
//!     let language_output = Language::English;
//!
//!     println!("{}: {}", language_input, input);
//!
//!     match translate(language_input, language_output, input) {
//!         Ok(output) => println!("{}: {}", language_output, output),
//!         Err(error) => eprintln!("Translation error: {}", error),
//!     };
//! }
//! ```
//!
//! Output:
//! ```
//! fr: Je te déteste!
//! en: I hate you!
//! ```
//!
//! Written with love, in Rust by Grant Handy.

use serde_json::Value;
use std::fmt;

/// Languages used for input and output of the translate function.
#[derive(Debug, Clone, Copy)]
pub enum Language {
    English,
    Arabic,
    Chinese,
    French,
    German,
    Italian,
    Portuguese,
    Russain,
    Spanish,
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Language::English => write!(f, "en"),
            Language::Arabic => write!(f, "ar"),
            Language::Chinese => write!(f, "zh"),
            Language::French => write!(f, "fr"),
            Language::German => write!(f, "de"),
            Language::Italian => write!(f, "it"),
            Language::Portuguese => write!(f, "pt"),
            Language::Russain => write!(f, "rs"),
            Language::Spanish => write!(f, "es"),
        }
    }
}

/// Errors that could be outputed by translate()
#[derive(Debug, Clone)]
pub enum TranslateError {
    HttpError(String),
    ParseError(String),
}

impl std::error::Error for TranslateError {}

impl fmt::Display for TranslateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TranslateError::HttpError(error) => {
                write!(f, "HTTP Request Error: {}", error.to_string())
            }
            TranslateError::ParseError(error) => {
                write!(f, "JSON Parsing Error: {}", error.to_string())
            }
        }
    }
}

/// Translate text between two languages.
pub fn translate(
    input_lang: Language,
    output_lang: Language,
    input: &str,
) -> Result<String, TranslateError> {
    let input_lang_str: String = match input_lang {
        Language::English => "en".to_string(),
        Language::Arabic => "ar".to_string(),
        Language::Chinese => "zh".to_string(),
        Language::French => "fr".to_string(),
        Language::German => "de".to_string(),
        Language::Italian => "it".to_string(),
        Language::Portuguese => "pt".to_string(),
        Language::Russain => "rs".to_string(),
        Language::Spanish => "es".to_string(),
    };

    let output_lang_str: String = match output_lang {
        Language::English => "en".to_string(),
        Language::Arabic => "ar".to_string(),
        Language::Chinese => "zh".to_string(),
        Language::French => "fr".to_string(),
        Language::German => "de".to_string(),
        Language::Italian => "it".to_string(),
        Language::Portuguese => "pt".to_string(),
        Language::Russain => "rs".to_string(),
        Language::Spanish => "es".to_string(),
    };

    let output = match translate_request(&input_lang_str, &output_lang_str, input) {
        Ok(data) => data,
        Err(error) => return Err(error),
    };

    Ok(output)
}

fn translate_request(
    source: &str,
    target: &str,
    q: &str,
) -> std::result::Result<String, TranslateError> {
    match ureq::post("https://libretranslate.com/translate").send_json(ureq::json!({
        "q": q,
        "source": source,
        "target": target,
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

            let text = match &parsed_json["translatedText"] {
                Value::String(text) => text,
                _ => {
                    return Err(TranslateError::ParseError(String::from(
                        "Unable to find translatedText in parsed JSON",
                    )))
                }
            };

            return Ok(text.to_string());
        }
        Err(error) => return Err(TranslateError::HttpError(error.to_string())),
    };
}
