use libretranslate::{translate, Language};

fn main() {
    let input = "Olá Mundo!";
    let language_input = Language::Portuguese;
    let language_output = Language::English;

    let output = match translate(language_input, language_output, input) {
        Ok(output) => output,
        Err(error) => panic!("Translation error: {}", error),
    };

    println!("{}: {}", language_input.pretty(), input);
    println!("{}: {}", language_output.pretty(), output);
}
