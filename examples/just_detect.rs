// Here we use the detect() function to detect the language of our text using whatlang.
// Really it's just an abstraction over whatlang, but it isn't a big deal because this makes it much easier for the user.

fn main() {
    let text = "Salut! Je dis en Francais.";
    let lang = libretranslate::detect(text).unwrap();

    println!("Text: {}", text);
    println!("Detected Language: {}", lang.as_pretty());
}