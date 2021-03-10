use libretranslate::Language;

// A demonstration of how to use Language with FromStr to parse str's into Languages.
fn main() {
    let chinese = "zh".parse::<Language>().unwrap();
    let french  = "FRENCH".parse::<Language>().unwrap();

    println!("\"zh\" parsed to pretty: {}", chinese.pretty());
    println!("\"FRENCH\" parsed to code: {}", french.code());
}