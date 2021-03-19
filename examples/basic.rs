use libretranslate::{translate, Language};

fn main() {
    let source = Language::French;
    let target = Language::English;
    let input = "le texte français.";

    let data = translate(Some(source), target, input).unwrap();

    println!("Input {}: {}", data.source.as_pretty(), data.input);
    println!("Output {}: {}", data.target.as_pretty(), data.output);
}