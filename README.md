# libretranslate-rs
A LibreTranslate API for Rust.

libretranslate-rs allows you to use open source machine translation in your projects through an easy to use API.

Using it is fairly simple:
```
use libretranslate::Language;

fn main() {
    let input = "Open Source Machine Translation";

    match libretranslate::translate(Language::English, Language::French, input) {
        Ok(output) => println!("Translation of {} into French: {}", input, output),
        Err(error) => println!("Translation error: {}", error),
    };
}
```

Written with love, in Rust.
