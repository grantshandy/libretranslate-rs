use libretranslate::{translate, Language};

fn main() {
    let input = "Je te déteste!";
    let language_input = Language::French;
    let language_output = Language::English;

    println!("{}: {}", language_input, input);

    match translate(language_input, language_output, input) {
        Ok(output) => println!("{}: {}", language_output, output),
        Err(error) => eprintln!("Translation error: {}", error),
    };
}
