// The trait `Translate` implements `AsRef<str>`, meaning that any `&str` or `String` can be translated into any other language.

use libretranslate::{Translate, Language};

fn main() {
    let text = "This is text, written on a computer, in English."
        .from_lang(Language::German)
        .to_lang(Language::English)
        .translate().unwrap();
    
    println!("{}", text);
}
