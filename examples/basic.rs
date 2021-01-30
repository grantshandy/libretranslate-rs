use libretranslate::Language;

fn main() {
    let input = "Open Source Machine Translation";

    match libretranslate::translate(Language::English, Language::French, input) {
        Ok(output) => println!("Translation of {} into French: {}", input, output),
        Err(error) => println!("Translation error: {}", error),
    };
}
