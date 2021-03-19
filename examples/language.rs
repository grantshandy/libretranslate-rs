use libretranslate::Language;

// Here we demonstrate that we can use Language::from() to create languages from unknown variables such as user input.
fn main() {
    let lang = Language::English;
    let lang_parse = "english".parse::<Language>().unwrap();


    assert_eq!(lang, lang_parse);
    assert_eq!("en", lang.as_code());
    assert_eq!("English", lang.as_pretty());
}