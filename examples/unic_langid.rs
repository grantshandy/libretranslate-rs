use libretranslate::{translate_url, Language};
use unic_langid::LanguageIdentifier;

#[tokio::main]
async fn main() {
    let source = Language::English;

    let li: LanguageIdentifier = "fr-FR".parse().expect("Failed to parse.");
    let target: Language = Language::from_unic_langid(li).unwrap();

    let input = "Amazing!";
    
    let data = translate_url(source, target, input, "https://libretranslate.de/", None).await.unwrap();

    println!("URL: \"{}\"", data.url);
    println!("Input {}: \"{}\"", data.source.as_pretty(), data.input);
    println!("Output {}: \"{}\"", data.target.as_pretty(), data.output);
}