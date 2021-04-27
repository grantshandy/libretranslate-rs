// USAGE:
// $ ./args <INPUT> <OUTPUT> <TRANSLATED TEXT>

// The translated text goes in quotation marks.


use libretranslate::{translate, Language};

#[tokio::main]
async fn main() {
    let args = &std::env::args().collect::<Vec<_>>();

    if args.len() != 4 {
        eprintln!("FORMAT: <INPUT LANGUAGE> <OUTPUT LANGUAGE> <INPUT TEXT>");
        std::process::exit(1);
    };

    let source = match args[1].parse::<Language>() {
        Ok(source) => source,
        Err(_) => {
            eprintln!("{} is not a valid language.", args[1]);
            std::process::exit(1);
        }
    };

    let target = match args[2].parse::<Language>() {
        Ok(source) => source,
        Err(_) => {
            eprintln!("{} is not a valid language.", args[2]);
            std::process::exit(1);
        }
    };

    let input = &args[3];

    match translate(source, target, input).await {
        Ok(data) => println!("{}", data.output),
        Err(error) => println!("Error: {}", error),
    };
}
