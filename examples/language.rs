use libretranslate::Language;

// Here we demonstrate that we can use Language::from() to create languages from unknown variables such as user input.
fn main() {
    let lang: Language = Language::English;
    let lang_parse: Language = "en".parse().unwrap();


    assert_eq!(lang, lang_parse);
    assert_eq!("en", lang.code());
    assert_eq!("English", lang.pretty())
}