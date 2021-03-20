// The trait `Translate` implements `AsRef<str>`, meaning that any `&str` or `String` can be translated into any other language.

use libretranslate::{Translate, Language};

fn main() {
    let text = "This is text, written on a computer, in English.".to_german_from(Language::English).unwrap();
    
    println!("{}", text);
}