# libretranslate-rs
A LibreTranslate API for Rust.
```
libretranslate = "0.2.1"
```

`libretranslate` allows you to use open source machine translation in your projects through an easy to use API that connects to the official [webpage](https://libretranslate.com/).

## Basic Example
All translations are done through the `translate()` function:
```rust
use libretranslate::{translate, Language};

fn main() {
    let source = Language::French;
    let target = Language::English;
    let input = "le texte français.";

    let data = translate(Some(source), target, input).unwrap();

    println!("Input {}: {}", data.source.as_pretty(), data.input);
    println!("Output {}: {}", data.target.as_pretty(), data.output);
}
```

Output:
```
Input French: le texte français.
Output English: the French text.
```

## Language Detection
`libretranslate` uses [`whatlang`](https://crates.io/crates/whatlang) to detect language so you can translate unknown languages into a target language of your choice.

`whatlang` isn't near perfect though, and for short sentences it can be very bad at detecting language. `whatlang` can detect more languages than `libretranslate` can translate, so if it detects your input as a language that `libretranslate` can't translate, the `translate` function will return a `TranslateError::DetectError`.

Here's a simple example.
```rust
use libretranslate::{Language, translate};

fn main() {
    let target = Language::English;
    let text = "le texte français.";

    let data = translate(None, target, text).unwrap();

    println!("Input {}: {}", data.source.as_pretty(), data.input);
    println!("Output {}: {}", data.target.as_pretty(), data.output);
}
```

Output:
```
Input French: le texte français.
Output English: the French text.
```

## Language Functionality
The `Language` enum has a lot of functionality so you can create a `Language` from all sorts of different user inputs.

You can return a `&str` with the language's name in English using `as_pretty()`, or the language's code using `as_code()`.

`Language` also implements `FromStr` so you can create a `Language` using text like "en", or "English" (case doesn't matter). You can do this by either using `Language::from()` or `.parse::<Language>()`.

Here's a simple example.
```rust
use libretranslate::Language;

fn main() {
    let english = Language::from("EnGlIsH").unwrap();
    let chinese = "zh".parse::<Language>().unwrap().as_pretty();
    let french  = "FRENCH".parse::<Language>().unwrap().as_code();

    println!("\"EnGlIsH\" parsed to code: {}", english);
    println!("\"zh\" parsed to pretty: {}", chinese);
    println!("\"FRENCH\" parsed to code: {}", french);
}
```

Output:
```
"EnGlIsH" parsed to code: en
"zh" parsed to pretty: Chinese
"FRENCH" parsed to code: fr
```

## String Methods
The trait `Translate` implements `AsRef<str>`, meaning that any `&str` or `String` can be translated into any other language. These methods use `whatlang`, so be careful that your text is clearly apart of a certain language and not vague/short.

Here's a simple example.
```rust
use libretranslate::Translate;

fn main() {
    let text = "Détecter une langue et un script par un texte donné.".to_english().unwrap();
    
    println!("{}", text);
}
```

Output:
```
detect a language and script by a given text.
```

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

Written in Rust, with love by Grant Handy.
