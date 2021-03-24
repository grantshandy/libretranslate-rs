// The trait `Translate` implements `AsRef<str>`, meaning that any `&str` or `String` can be translated into any other language.

use libretranslate::{Language, Translate};

fn main() {
    let text = "This is text, written on a computer, in English."
        .to_lang(Language::German)
        .from_lang(Language::English);

    println!("text: \"{}\"", text.text);
    println!("source: {}", text.source.unwrap());
    println!("target: {}", text.target);
    println!("output: \"{}\"", text.translate().unwrap());
}
