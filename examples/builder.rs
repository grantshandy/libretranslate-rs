use libretranslate::{TranslationBuilder, Language};

#[tokio::main]
async fn main() {
    let builder = TranslationBuilder::new()
        .text("le texte francais")
        .from_lang(Language::French)
        .to_lang(Language::Italian)
        .url("https://libretranslate.de/")
        .translate()
        .await
        .unwrap();

    println!("{}", builder);
}