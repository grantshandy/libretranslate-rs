// The trait `Translate` implements `AsRef<str>`, meaning that any `&str` or `String` can be translated into any other language.

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
