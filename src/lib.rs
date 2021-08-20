//! # libretranslate-rs
//!! [![Crates.io](https:img.shields.io/crates/v/libretranslate.svg)](https://crates.io/crates/libretranslate)
//! [![Crates.io](https://img.shields.io/crates/d/libretranslate)](https://crates.io/crates/libretranslate)
//! [![API](https://docs.rs/libretranslate/badge.svg)](https://docs.rs/libretranslate)
//! [![Gitpod ready-to-code](https://img.shields.io/badge/Gitpod-ready--to--code-blue?logo=gitpod)](https://gitpod.io/#https://github.com/grantshandy/libretranslate-rs)
//! ![GitHub Workflow Status](https://img.shields.io/github/workflow/status/grantshandy/libretranslate-rs/Rust)
//!
//! A LibreTranslate API client for Rust.
//! ```
//! libretranslate = "0.5.1"
//! ```
//!
//! `libretranslate` allows you to use open source machine translation in your projects through an easy to use API that connects to the official [webpage](https://libretranslate.com/).
//!
//! ## Basic Example
//! `libretranslate` is an async library, so you'll have to use an async runtime like [`tokio`](https://crates.io/crates/tokio) or [`async-std`](https://crates.io/crates/async-std).
//!
//! All translations are done through the [`translate`](crate::translate) function:
//! ```rust
//! use libretranslate::{translate, Language};
//!
//! #[tokio::main]
//! async fn main() {
//!     let source = Language::French;
//!     let target = Language::English;
//!     let input = "Le texte français.";
//!
//!     let data = translate(source, target, input, None).await.unwrap();
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
//! [See In Examples Folder](https://github.com/grantshandy/libretranslate-rs/blob/main/examples/basic.rs)
//!
//! ## Language Detection
//! Here's a simple example.
//! ```rust
//! use libretranslate::{translate, Language};
//!
//! #[tokio::main]
//! async fn main() {
//!     let target = Language::English;
//!     let text = "le texte français.";
//!
//!     let data = translate(Language::Detect, target, text, None).await.unwrap();
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
//! [See In Examples Folder](https://github.com/grantshandy/libretranslate-rs/blob/main/examples/detect.rs)
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
//! let lang = Language::English;
//! let lang_parse = "english".parse::<Language>().unwrap();
//!
//! assert_eq!(lang, lang_parse);
//! assert_eq!("en", lang.as_code());
//! assert_eq!("English", lang.as_pretty());
//! ```
//!
//! [See In Examples Folder](https://github.com/grantshandy/libretranslate-rs/blob/main/examples/language.rs)
//!
//! ## String Methods
//! The trait `Translate` implements [`AsRef<str>`](https://doc.rust-lang.org/std/convert/trait.AsRef.html), meaning that any `&str` or `String` can be translated into any other language.
//!
//! Here's a simple example.
//! ```rust
//! use libretranslate::{Language, Translate};
//!
//! #[tokio::main]
//! async fn main() {
//!     let text = "This is text, written on a computer, in English."
//!         .to_lang(Language::German)
//!         .from_lang(Language::English)
//!         .translate()
//!         .await
//!         .unwrap();
//!
//!     println!("output: \"{}\"", text);
//! }
//! ```
//!
//! Output:
//! ```
//! Output: "Dies ist Text, geschrieben auf einem Computer, in Englisch."
//! ```
//!
//! [See In Examples Folder](https://github.com/grantshandy/libretranslate-rs/blob/main/examples/method.rs)
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
//!! Written with love, in Rust by [Grant Handy](mailto://grantshandy@gmail.com).

use serde_json::Value;

/// Data that is output by the [`translate`](translate) function.
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Translation {
    pub url: String,
    pub source: Language,
    pub target: Language,
    pub input: String,
    pub output: String,
}

/// Translate text between two [`Language`](Language).
pub async fn translate<T: AsRef<str>>(
    source: Language,
    target: Language,
    input: T,
    key: Option<T>,
) -> Result<Translation, TranslateError> {
    let url = "https://libretranslate.com/";

    let key: Option<String> = key.map(|data| data.as_ref().to_string());

    let data = translate_url(source, target, input.as_ref(), url, key).await?;

    Ok(data)
}

/// Translate using a custom URL.
pub async fn translate_url<T: AsRef<str>>(
    source: Language,
    target: Language,
    input: T,
    url: T,
    key: Option<String>,
) -> Result<Translation, TranslateError> {
    let complete_url: String;

    if url.as_ref().ends_with('/') {
        complete_url = format!("{}translate", url.as_ref());
    } else {
        complete_url = format!("{}/translate", url.as_ref());
    };

    if input.as_ref().chars().count() >= 5000 {
        return Err(TranslateError::LengthError);
    };

    let data: Value = match key {
        Some(key) => {
            serde_json::json!({
                "q": input.as_ref(),
                "source": source.as_code(),
                "target": target.as_code(),
                "api_key": key,
            })
        }
        None => {
            serde_json::json!({
                "q": input.as_ref(),
                "source": source.as_code(),
                "target": target.as_code(),
            })
        }
    };

    let body = match surf::http::Body::from_json(&data) {
        Ok(data) => data,
        Err(error) => return Err(TranslateError::HttpError(error.to_string())),
    };

    let url = complete_url.clone();

    let res = match surf::post(complete_url).body(body).recv_string().await {
        Ok(data) => data,
        Err(error) => return Err(TranslateError::HttpError(error.to_string())),
    };

    let parsed_json: Value = match serde_json::from_str(&res) {
        Ok(parsed_json) => parsed_json,
        Err(error) => {
            return Err(TranslateError::ParseError(error.to_string()));
        }
    };

    if let Value::String(error) = &parsed_json["error"] {
        return Err(TranslateError::ParseError(error.to_string()));
    }

    let output = match &parsed_json["translatedText"] {
        Value::String(output) => output,
        _ => {
            return Err(TranslateError::ParseError(String::from(
                "Unable to find translatedText in parsed JSON",
            )))
        }
    };

    let input = input.as_ref().to_string();
    let output = output.to_string();

    Ok(Translation {
        url,
        source,
        target,
        input,
        output,
    })
}

use std::str::FromStr;

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

    /// Create a Language from a [`LanguageIdentifier`](unic_langid::LanguageIdentifier).
    #[cfg(feature = "unicode_langid")]
    pub fn from_unic_langid(s: unic_langid::LanguageIdentifier) -> Result<Self, LanguageError> {
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
            "auto" => Ok(Language::Detect),
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

/// Errors that could be outputed by a [`Language`](Language).
#[derive(Debug, Clone, PartialEq, Hash)]
pub enum LanguageError {
    FormatError(String),
}

impl std::error::Error for LanguageError {}

impl std::fmt::Display for LanguageError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            LanguageError::FormatError(error) => {
                write!(f, "Unknown Language: {}", error)
            }
        }
    }
}

/// Errors that could be outputed by [`translate`](crate::translate).
#[derive(Debug, Clone, PartialEq, Hash)]
pub enum TranslateError {
    HttpError(String),
    ParseError(String),
    DetectError,
    LengthError,
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
            TranslateError::LengthError => {
                write!(f, "Requested text is too long")
            }
        }
    }
}

/// A struct created by a [`Translate`](Translate) that can be translated using the translate method.
pub struct Query<'a> {
    pub url: &'a str,
    pub text: &'a str,
    pub source: Language,
    pub target: Language,
}

impl<'a> Query<'a> {
    pub fn to_lang(mut self, language: Language) -> Query<'a> {
        self.target = language;
        self
    }

    pub fn from_lang(mut self, language: Language) -> Query<'a> {
        self.source = language;
        self
    }

    pub fn url(mut self, url: &'a str) -> Query {
        self.url = url;
        self
    }

    pub async fn translate(self) -> Result<String, TranslateError> {
        let res = crate::translate_url(self.source, self.target, self.text, self.url, None).await?;
        Ok(res.output)
    }
}

/// Translate text from a [`String`](std::string::String) or [`str`](std::str) (anything that implements [`AsRef<str>`](std::convert::AsRef)).
pub trait Translate {
    fn to_lang(&self, language: Language) -> Query;
    fn from_lang(&self, language: Language) -> Query;
}

impl<T> Translate for T
where
    T: AsRef<str>,
{
    fn to_lang(&self, language: Language) -> Query {
        Query {
            url: "https://libretranslate.com/",
            text: self.as_ref(),
            source: Language::Detect,
            target: language,
        }
    }

    fn from_lang(&self, language: Language) -> Query {
        Query {
            url: "https://libretranslate.com/",
            text: self.as_ref(),
            source: language,
            target: Language::default(),
        }
    }
}

/// Build Translations more verbosely.
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct TranslationBuilder {
    pub url: String,
    pub source: Language,
    pub target: Language,
    pub input: String,
    key: Option<String>,
}

impl TranslationBuilder {
    pub fn new() -> Self {
        Self {
            url: String::from("https://libretranslate.com/"),
            source: Language::Detect,
            target: Language::default(),
            input: String::new(),
            key: None,
        }
    }

    pub fn url<T: AsRef<str>>(mut self, url: T) -> Self {
        self.url = url.as_ref().to_string();
        self
    }

    pub fn from_lang(mut self, lang: Language) -> Self {
        self.source = lang;
        self
    }

    pub fn to_lang(mut self, lang: Language) -> Self {
        self.target = lang;
        self
    }

    pub fn text<T: AsRef<str>>(mut self, text: T) -> Self {
        self.input = text.as_ref().to_string();
        self
    }

    pub fn key<T: AsRef<str>>(mut self, key: T) -> Self {
        self.key = Some(key.as_ref().to_string());
        self
    }

    pub async fn translate(mut self) -> Result<Translation, TranslateError> {
        if self.input.is_empty() {
            return Ok(Translation {
                url: self.url,
                source: self.source,
                target: self.target,
                input: self.input,
                output: String::new(),
            });
        };

        let data = translate_url(
            self.source,
            self.target,
            self.input.clone(),
            self.url.clone(),
            self.key,
        )
        .await?;

        self.source = data.source;
        self.target = data.target;

        Ok(Translation {
            url: self.url,
            source: self.source,
            target: self.target,
            input: self.input,
            output: data.output,
        })
    }
}
impl Default for TranslationBuilder {
    fn default() -> Self {
        Self::new()
    }
}
