use libretranslate::{translate, Language};

#[tokio::main]
async fn main() {
    let args = &std::env::args().collect::<Vec<_>>();

    let source = Language::English;
    let target = Language::Japanese;
    let input = "what is the problem!";

    let data = translate(source, target, input, Some(&args[1])).await.unwrap();

    println!("Input {}: {}", data.source.as_pretty(), data.input);
    println!("Output {}: {}", data.target.as_pretty(), data.output);
}
