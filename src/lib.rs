//! # libretranslate-rs
//! A LibreTranslate API for Rust.
//! ```
//! libretranslate = "0.2.2"
//! ```
//!
//! `libretranslate` allows you to use open source machine translation in your projects through an easy to use API that connects to the official [webpage](https://!libretranslate.com/).
//!
//! ## Basic Example
//! All translations are done through the `translate()` function:
//! ```rust
//! use libretranslate::{translate, Language};
//!
//! fn main() {
//!     let source = Language::French;
//!     let target = Language::English;
//!     let input = "le texte français.";
//!
//!     let data = translate(Some(source), target, input).unwrap();
//!
//!     println!("Input {}: {}", data.source.as_pretty(), data.input);
//!     println!("Output {}: {}", data.target.as_pretty(), data.output);
//! }
//! ```
//!
//! Output:
//! ```
//! Input French: le texte français.
//! Output English: the French text.
//! ```
//!
//! ## Language Detection
//! `libretranslate` uses [`whatlang`](https://!crates.io/crates/whatlang) to detect language so you can translate unknown languages into a target language of your choice.
//!
//! `whatlang` isn't perfect though, and for short sentences it can be very bad at detecting language. `whatlang` can detect more languages than `libretranslate` can translate, so if it detects your input as a language that `libretranslate` can't translate, the `translate` function will return a `TranslateError::DetectError`.
//!
//! Here's a simple example.
//! ```rust
//! use libretranslate::{Language, translate};
//!
//! fn main() {
//!     let target = Language::English;
//!     let text = "le texte français.";
//!
//!     let data = translate(None, target, text).unwrap();
//!
//!     println!("Input {}: {}", data.source.as_pretty(), data.input);
//!     println!("Output {}: {}", data.target.as_pretty(), data.output);
//! }
//! ```
//!
//! Output:
//! ```
//! Input French: le texte français.
//! Output English: the French text.
//! ```
//!
//! ## Language Functionality
//! The `Language` enum has a lot of functionality so you can create a `Language` from all sorts of different user inputs.
//!
//! You can return a `&str` with the language's name in English using `as_pretty()`, or the language's code using `as_code()`.
//!
//! `Language` also implements `FromStr` so you can create a `Language` using text like "en", or "English" (case doesn't matter). You can do this by either using `Language::from()` or `.parse::<Language>()`.
//!
//! Here's a simple example.
//! ```rust
//! use libretranslate::Language;
//!
//! fn main() {
//!     let english = Language::from("EnGlIsH").unwrap();
//!     let chinese = "zh".parse::<Language>().unwrap().as_pretty();
//!     let french  = "FRENCH".parse::<Language>().unwrap().as_code();
//!
//!     println!("\"EnGlIsH\" parsed to code: {}", english);
//!     println!("\"zh\" parsed to pretty: {}", chinese);
//!     println!("\"FRENCH\" parsed to code: {}", french);
//! }
//! ```
//!
//! Output:
//! ```
//! "EnGlIsH" parsed to code: en
//! "zh" parsed to pretty: Chinese
//! "FRENCH" parsed to code: fr
//! ```
//!
//! ## String Methods
//! The trait `Translate` implements `AsRef<str>`, meaning that any `&str` or `String` can be translated into any other language. These methods use `whatlang`, so be careful that your text is clearly apart of a certain language and not vague/short.
//!
//! Here's a simple example.
//! ```rust
//! use libretranslate::Translate;
//!
//! fn main() {
//!     let text = "Détecter une langue et un script par un texte donné.".to_english().unwrap();
//!
//!     println!("{}", text);
//! }
//! ```
//!
//! Output:
//! ```
//! detect a language and script by a given text.
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
//! Written in Rust, with love by [Grant Handy](mailto://!grantshandy@gmail.com).

use serde_json::Value;
use std::str::FromStr;
use whatlang::Lang;

/// Languages that can used for input and output of the [`translate`] function.
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
#[derive(Debug, Clone, PartialEq)]
pub enum TranslateError {
    HttpError(String),
    ParseError(String),
    DetectError,
    Length,
}

impl std::error::Error for TranslateError {}

impl std::fmt::Display for TranslateError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TranslateError::HttpError(error) => {
                write!(f, "HTTP request error: {}", error.to_string())
            }
            TranslateError::ParseError(error) => {
                write!(f, "JSON parsing error: {}", error.to_string())
            }
            TranslateError::DetectError => {
                write!(f, "Language detection error")
            }
            TranslateError::Length => {
                write!(f, "Requested text is too long")
            }
        }
    }
}

/// Data that is output by the [`translate`] function.
pub struct Translator {
    pub source: Language,
    pub target: Language,
    pub input: String,
    pub output: String,
}

/// Translate text between two languages.
pub fn translate(source: Option<Language>, target: Language, input: &str) -> Result<Translator, TranslateError> {
    if input.chars().count() >= 5000 {
        return Err(TranslateError::Length);
    };

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

/// A trait that lets you convert [`AsRef<str>`] into translated text.
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