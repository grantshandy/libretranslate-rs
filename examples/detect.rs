// `libretranslate` uses [`whatlang`](https://crates.io/crates/whatlang). to detect language so you can translate unknown languages into a target language of your choice.
// `whatlang` isn't perfect though, and for short sentences it can be very bad at detecting language. `whatlang` can detect more languages than `libretranslate` can translate, so if it detects your input as a language that `libretranslate` can't translate, the `translate` function will return a `TranslateError::DetectError`.

use libretranslate::{translate, Language};

#[tokio::main]
async fn main() {
    let target = Language::English;
    let text = "le texte français.";

    let data = translate(None, target, text).await.unwrap();

    println!("Input {}: {}", data.source.as_pretty(), data.input);
    println!("Output {}: {}", data.target.as_pretty(), data.output);
}
