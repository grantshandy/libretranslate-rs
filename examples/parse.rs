use libretranslate::Language;

fn main() {
    let chinese = "zh".parse::<Language>().unwrap().as_pretty();
    let french  = "FRENCH".parse::<Language>().unwrap().as_code();
    let english = Language::from("EnGlIsH").unwrap();

    println!("\"EnGlIsH\" parsed to code: {}", english);
    println!("\"zh\" parsed to pretty: {}", chinese);
    println!("\"FRENCH\" parsed to code: {}", french);
}