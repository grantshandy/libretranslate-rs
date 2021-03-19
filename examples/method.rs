// The trait `Translate` implements `AsRef<str>`, meaning that any `&str` or `String` can be translated into any other language. These methods use `whatlang`, so be careful that your text is clearly apart of a certain language and not vague/short.

use libretranslate::Translate;

fn main() {
    let text = "Détecter une langue et un script par un texte donné.".to_english().unwrap();
    
    println!("{}", text);
}