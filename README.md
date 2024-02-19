# libretranslate-rs
[![Crates.io](https://img.shields.io/crates/v/libretranslate.svg)](https://crates.io/crates/libretranslate)
[![Crates.io](https://img.shields.io/crates/d/libretranslate)](https://crates.io/crates/libretranslate)
[![API](https://docs.rs/libretranslate/badge.svg)](https://docs.rs/libretranslate)
[![Gitpod ready-to-code](https://img.shields.io/badge/Gitpod-ready--to--code-blue?logo=gitpod)](https://gitpod.io/#https://github.com/grantshandy/libretranslate-rs)
![GitHub Workflow Status](https://img.shields.io/github/workflow/status/grantshandy/libretranslate-rs/Rust)

A LibreTranslate API client for Rust.
```
libretranslate = "0.5.1"
```

`libretranslate` allows you to use open source machine translation in your projects through an easy to use API that connects to the official [webpage](https://libretranslate.com/).

### Update for Oct, 2021
As of now, [libretranslate.com](https://libretranslate.com) hasn't implemented rate limiting yet, so you must use an alternate instance or have an API key.

Here are some that work:
- [translate.terraprint.co](https://translate.terraprint.co/) (Free)
- [libretranslate.eownerdead.dedyn.io](https://libretranslate.eownerdead.dedyn.io/) (Free)
- [libretranslate.com](https://libretranslate.com) (Paid)

You'll need to use a Builder struct or a String method and specify the url to change to an instance that doesn't require an API key.

## Basic Example
`libretranslate` is an async library, so you'll have to use an async runtime like [`tokio`](https://crates.io/crates/tokio) or [`async-std`](https://crates.io/crates/async-std).

All translations are done through the `translate` function:
```rust
use libretranslate::{translate, Language};

#[tokio::main]
async fn main() {
    let source = Language::French;
    let target = Language::English;
    let input = "Le texte français.";

    let data = translate(source, target, input, None).await.unwrap();

    println!("Input {}: {}", data.source.as_pretty(), data.input);
    println!("Output {}: {}", data.target.as_pretty(), data.output);
}
```

Output:
```
Input French: le texte français.
Output English: the French text.
```

[See In Examples Folder](https://github.com/grantshandy/libretranslate-rs/blob/main/examples/basic.rs)

## Language Detection
Here's a simple example.
```rust
use libretranslate::{translate, Language};

#[tokio::main]
async fn main() {
    let target = Language::English;
    let text = "le texte français.";

    let data = translate(Language::Detect, target, text, None).await.unwrap();

    println!("Input {}: {}", data.source.as_pretty(), data.input);
    println!("Output {}: {}", data.target.as_pretty(), data.output);
}
```

Output:
```
Input French: le texte français.
Output English: the French text.
```

[See In Examples Folder](https://github.com/grantshandy/libretranslate-rs/blob/main/examples/detect.rs)

## Language Functionality
The `Language` enum has a lot of functionality so you can create a `Language` from all sorts of different user inputs.

You can return a `&str` with the language's name in English using `as_pretty()`, or the language's code using `as_code()`.

`Language` also implements `FromStr` so you can create a `Language` using text like "en", or "English" (case doesn't matter). You can do this by either using `Language::from()` or `.parse::<Language>()`.

Here's a simple example.
```rust
use libretranslate::Language;

fn main() {
    let lang = Language::English;
    let lang_parse = "english".parse::<Language>().unwrap();

    assert_eq!(lang, lang_parse);
    assert_eq!("en", lang.as_code());
    assert_eq!("English", lang.as_pretty());
}
```

[See In Examples Folder](https://github.com/grantshandy/libretranslate-rs/blob/main/examples/language.rs)

## String Methods
The trait `Translate` implements [`AsRef<str>`](https://doc.rust-lang.org/std/convert/trait.AsRef.html), meaning that any `&str` or `String` can be translated into any other language. 

Here's a simple example.
```rust
use libretranslate::{Language, Translate};

#[tokio::main]
async fn main() {
    let text = "This is text, written on a computer, in English."
        .to_lang(Language::German)
        .from_lang(Language::English)
        .translate()
        .await
        .unwrap();

    println!("output: \"{}\"", text);
}
```

Output:
```
Output: "Dies ist Text, geschrieben auf einem Computer, in Englisch."
```

[See In Examples Folder](https://github.com/grantshandy/libretranslate-rs/blob/main/examples/method.rs)

## Available Languages
- English
- Arabic
- Chinese
- French
- German
- Italian
- Japanese
- Portuguese
- Russian
- Spanish
- Polish
