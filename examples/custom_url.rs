use libretranslate::{translate_url, Language};

#[tokio::main]
async fn main() {
    let source = Language::French;
    let target = Language::English;
    let input = "Le texte français";

    let data = translate_url(source, target, input, "https://libretranslate.de/").await.unwrap();

    println!("URL: \"{}\"", data.url);
    println!("Input {}: \"{}\"", data.source.as_pretty(), data.input);
    println!("Output {}: \"{}\"", data.target.as_pretty(), data.output);
}
