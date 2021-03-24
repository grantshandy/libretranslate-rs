// The trait `Translate` implements `AsRef<str>`, meaning that any `&str` or `String` can be translated into any other language.

use libretranslate::{Language, Translate};

fn main() {
    let text = "This is text, written on a computer, in English."
        .from_lang(Language::English)
        .to_lang(Language::French)
        .translate()
        .unwrap();

    println!("{}", text);
}
