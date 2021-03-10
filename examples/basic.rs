use libretranslate::{Translator, Language};

fn main() {
    let input = "Olá Mundo!";
    let source = Language::Portuguese;
    let target = Language::English;

    match Translator::translate(source, target, input) {
        Ok(data) => println!("{}", data.output),
        Err(error) => panic!("{}", error),
    };
}
