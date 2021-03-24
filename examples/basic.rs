// All translations are done through the `translate()` function.

use libretranslate::{translate, Language};

fn main() {
    let source = Language::French;
    let target = Language::English;
    let input = "Le texte français.";

    let data = translate(Some(source), target, input).unwrap();

    println!("Input {}: {}", data.source.as_pretty(), data.input);
    println!("Output {}: {}", data.target.as_pretty(), data.output);
}
