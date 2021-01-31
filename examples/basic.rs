use libretranslate::Language;

fn main() {
    let input = "Je t'deteste.";
    println!("{}", input);

    match libretranslate::translate(Language::French, Language::English, input) {
        Ok(output) => println!("{}", output),
        Err(error) => println!("Translation error: {}", error),
    };
}
