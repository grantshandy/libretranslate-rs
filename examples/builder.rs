use libretranslate::{Language, TranslationBuilder};

#[tokio::main]
async fn main() {
    let builder = TranslationBuilder::new()
        .text("le texte francais")
        .from_lang(Language::French)
        .to_lang(Language::Italian)
        .url("https://libretranslate.de/")
        // .key("YOUR-OWN-KEY")
        .translate()
        .await
        .unwrap();

    println!("{}", builder.output);
}
