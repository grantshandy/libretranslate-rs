use libretranslate::{Translator, Language};

fn main() {
    let source = Language::from("portuguese").unwrap();
    let target = Language::Japanese;
    let input = "Olá Mundo!";
    let output = Translator::translate(source, target, input).unwrap().output;

    println!("Input {}: {}", source.pretty(), input);
    println!("Output {}: {}", target.pretty(), output);
}