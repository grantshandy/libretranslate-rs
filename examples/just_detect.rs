fn main() {
    let text = "Salut! Je dis en Francais.";
    let lang = libretranslate::detect(text).unwrap();

    println!("Text: {}", text);
    println!("Detected Language: {}", lang.as_pretty());
}