use clap::{crate_version, App, Arg};
use libretranslate::Language;

// A simple CLI application for getting the city and country that an IP is located in.
fn main() {
    // Set CLI application details through clap.
    let matches = App::new("ipgeo")
        .version(crate_version!())
        .author("Grant H. <grantshandy@gmail.com>")
        .about("Translates text from one language to another")
        .arg(
            Arg::with_name("TEXT")
                .help("what text to translate")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("input")
                .long("input")
                .short("i")
                .help("choose what language to translate from")
                .takes_value(true)
                .required(true)
                .multiple(false)
                .possible_values(&["en", "ar", "zh", "fr", "de", "it", "pt", "rs", "es"]),
        )
        .arg(
            Arg::with_name("output")
                .long("output")
                .short("o")
                .help("choose what language to translate to")
                .takes_value(true)
                .required(true)
                .multiple(false)
                .possible_values(&["en", "ar", "zh", "fr", "de", "it", "pt", "rs", "es"]),
        )
        .arg(
            Arg::with_name("verbose")
                .long("verbose")
                .short("v")
                .help("run with verbose output")
                .required(false)
                .takes_value(false),
        )
        .get_matches();

    let input_language: Language = match matches.value_of("input") {
        Some("en") => Language::English,
        Some("ar") => Language::Arabic,
        Some("zh") => Language::Chinese,
        Some("fr") => Language::French,
        Some("de") => Language::German,
        Some("it") => Language::Italian,
        Some("pt") => Language::Portuguese,
        Some("rs") => Language::Russain,
        Some("es") => Language::Spanish,
        Some(&_) => {
            eprintln!("DIFFERENT VALUE FOR INPUT LANGUAGE... EXTREMELY UNEXPECTED!");
            std::process::exit(1);
        }
        None => {
            eprintln!("NO VALUE FOR INPUT LANGUAGE... EXTREMELY UNEXPECTED!");
            std::process::exit(1);
        }
    };

    let output_language: Language = match matches.value_of("output") {
        Some("en") => Language::English,
        Some("ar") => Language::Arabic,
        Some("zh") => Language::Chinese,
        Some("fr") => Language::French,
        Some("de") => Language::German,
        Some("it") => Language::Italian,
        Some("pt") => Language::Portuguese,
        Some("rs") => Language::Russain,
        Some("es") => Language::Spanish,
        Some(&_) => {
            eprintln!("DIFFERENT VALUE FOR OUTPUT LANGUAGE... EXTREMELY UNEXPECTED!");
            std::process::exit(1);
        }
        None => {
            eprintln!("NO VALUE FOR OUTPUT LANGUAGE... EXTREMELY UNEXPECTED!");
            std::process::exit(1);
        }
    };

    let input_lang_str: String = match matches.value_of("input") {
        Some(data) => data.to_string(),
        None => {
            eprintln!("NO VALUE FOR INPUT LANGUAGE... EXTREMELY UNEXPECTED!");
            std::process::exit(1);
        }
    };

    let output_lang_str: String = match matches.value_of("output") {
        Some(data) => data.to_string(),
        None => {
            eprintln!("NO VALUE FOR OUTPUT LANGUAGE... EXTREMELY UNEXPECTED!");
            std::process::exit(1);
        }
    };

    let text = match matches.value_of("TEXT") {
        Some(text) => text,
        None => {
            eprintln!("NO VALUE FOR TEXT... EXTREMELY UNEXPECTED!");
            std::process::exit(1);
        }
    };

    match libretranslate::translate(input_language, output_language, text) {
        Ok(data) => {
            if matches.is_present("verbose") {
                println!("{} source: \"{}\"", input_lang_str, text);
                println!("{} output: \"{}\"", output_lang_str, data);
            } else {
                println!("{}", data);
            };
        }
        Err(error) => eprintln!("Translation error: {}", error),
    };
}
