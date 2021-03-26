// USAGE:
// $ ./args <INPUT> <OUTPUT> <TRANSLATED TEXT>

// The translated text goes in quotation marks.


use libretranslate::{translate, Language};

#[tokio::main]
async fn main() {
    let args = &std::env::args().collect::<Vec<_>>();

    let source = args[1].parse::<Language>().unwrap();
    let target = args[2].parse::<Language>().unwrap();
    let input = &args[3];

    let data = translate(Some(source), target, input).await.unwrap();

    println!("Input {}: {}", data.source.as_pretty(), data.input);
    println!("Output {}: {}", data.target.as_pretty(), data.output);
}
