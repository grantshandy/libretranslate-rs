// The `Language` enum has a lot of functionality so you can create a `Language` from all sorts of different user inputs.

// You can return a `&str` with the language's name in English using `as_pretty()`, or the language's code using `as_code()`.

// `Language` also implements `FromStr` so you can create a `Language` using text like "en", or "English" (case doesn't matter). You can do this by either using `Language::from()` or `.parse::<Language>()`.

use libretranslate::Language;

fn main() {
    let lang = Language::English;
    let lang_parse = "english".parse::<Language>().unwrap();

    assert_eq!(lang, lang_parse);
    assert_eq!("en", lang.as_code());
    assert_eq!("English", lang.as_pretty());
}
