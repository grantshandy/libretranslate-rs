//! # libretranslate-rs
//! A LibreTranslate API client for Rust.
//! ```
//! libretranslate = "0.2.7"
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
//! [See In Examples Folder](https://github.com/DefunctLizard/libretranslate-rs/blob/main/examples/basic.rs)
//!
//! ## Language Detection
//! `libretranslate` uses [`whatlang`](https://crates.io/crates/whatlang) to detect language so you can translate unknown languages into a target language of your choice.
//!
//! `whatlang` isn't perfect though, and for short sentences it can be very bad at detecting language. `whatlang` can detect more languages than `libretranslate` can translate, so if it detects your input as a language that `libretranslate` can't translate, the `translate` function will return a `TranslateError::DetectError`.
//!
//! Here's a simple example.
//! ```rust
//! use libretranslate::{translate, Language};
//!
//! #[tokio::main]
//! async fn main() {
//!     let target = Language::English;
//!     let text = "le texte français.";
//!
//!     let data = translate(None, target, text).await.unwrap();
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
//! [See In Examples Folder](https://github.com/DefunctLizard/libretranslate-rs/blob/main/examples/detect.rs)
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
//!     let lang = Language::English;
//!     let lang_parse = "english".parse::<Language>().unwrap();
//!
//!     assert_eq!(lang, lang_parse);
//!     assert_eq!("en", lang.as_code());
//!     assert_eq!("English", lang.as_pretty());
//! }
//! ```
//!
//! [See In Examples Folder](https://github.com/DefunctLizard/libretranslate-rs/blob/main/examples/parse.rs)
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
//!     println!("Output: \"{}\"", text);
//! }
//!
//! ```
//!
//! Output:
//! ```
//! Output: "Dies ist Text, geschrieben auf einem Computer, in Englisch."
//! ```
//!
//! [See In Examples Folder](https://github.com/DefunctLizard/libretranslate-rs/blob/main/examples/method.rs)
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
//! Written in Rust, with love by [Grant Handy](mailto://grantshandy@gmail.com).

pub mod error;
pub mod languages;
pub mod traits;

use serde_json::Value;
use whatlang::{Lang, Detector};

pub use error::{LanguageError, TranslateError};
pub use languages::Language;
pub use traits::{Query, Translate};

/// Data that is output by the [`translate`](crate::translate) function.
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Translation {
    pub source: Language,
    pub target: Language,
    pub input: String,
    pub output: String,
}

/// Translate text between two [`Language`](crate::languages::Language).
pub async fn translate<T: AsRef<str>>(
    source: Option<Language>,
    target: Language,
    input: T,
) -> Result<Translation, TranslateError> {
    if input.as_ref().chars().count() >= 5000 {
        return Err(TranslateError::LengthError);
    };

    let source = match source {
        Some(data) => data,
        None => {
            match detect(input.as_ref()) {
                Ok(data) => data,
                Err(error) => return Err(error),
            }
        }
    };

    let data = match get_raw_data(source, target, input.as_ref()).await {
        Ok(data) => data,
        Err(error) => return Err(TranslateError::HttpError(error.to_string())),
    };
  
    let parsed_json: Value = match serde_json::from_str(&data) {
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

    let input = input.as_ref().to_string();
    let output = output.to_string();

    return Ok(Translation {
        source,
        target,
        input,
        output,
    });
}

async fn get_raw_data(source: Language, target: Language, input: &str) -> Result<String, surf::http::Error> {
    let uri = "https://libretranslate.com/translate";

    let data = serde_json::json!({
        "q": input,
        "source": source.as_code(),
        "target": target.as_code(),
    });

    println!("Running HTTP Post Request!");
    println!("URL: {}", uri);
    println!("JSON POST Body:\n{}", data.to_string());
    let res = surf::post(uri)
        .body(surf::http::Body::from_json(&data)?)
        .recv_string().await?;

    Ok(res)
}

pub fn detect<T: AsRef<str>>(text: T) -> Result<Language, TranslateError> {
    let allowlist = vec![Lang::Eng, Lang::Ara, Lang::Fra, Lang::Deu, Lang::Ita, Lang::Por, Lang::Rus, Lang::Spa, Lang::Jpn];

    let detector = Detector::with_allowlist(allowlist);

    let info = match detector.detect_lang(text.as_ref()) {
        Some(data) => data,
        None => return Err(TranslateError::DetectError),
    };

    let info = match info {
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
    };

    return Ok(info);
}