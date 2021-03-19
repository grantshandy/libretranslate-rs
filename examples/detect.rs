use libretranslate::{Language, translate};

fn main() {
    let target = Language::English;
    let text = "le texte français.";

    let data = translate(None, target, text).unwrap();

    println!("Input {}: {}", data.source.as_pretty(), data.input);
    println!("Output {}: {}", data.target.as_pretty(), data.output);
}