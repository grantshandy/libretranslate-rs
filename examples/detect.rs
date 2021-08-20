use libretranslate::{translate, Language};

#[tokio::main]
async fn main() {
    let target = Language::English;
    let text = "le texte français.";

    let data = translate(Language::Detect, target, text, None)
        .await
        .unwrap();

    println!("Input {}: {}", data.source.as_pretty(), data.input);
    println!("Output {}: {}", data.target.as_pretty(), data.output);
}
