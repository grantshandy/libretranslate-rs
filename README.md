# libretranslate-rs
A LibreTranslate API for Rust.
```
libretranslate = "0.1.3"
```

libretranslate-rs allows you to use open source machine translation in your projects through an easy to use API that connects to the official webpage.

Using it is fairly simple:

```rust
use libretranslate::{translate, Language};

fn main() {
    let input = "Je te déteste!";
    let language_input = Language::French;
    let language_output = Language::English;

    println!("{}: {}", language_input, input);

    match translate(language_input, language_output, input) {
        Ok(output) => println!("{}: {}", language_output, output),
        Err(error) => eprintln!("Translation error: {}", error),
    };
}
```

Output:
```
fr: Je te déteste!
en: I hate you!
```

Written with love, in Rust by Grant Handy.
