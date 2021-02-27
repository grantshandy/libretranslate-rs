# libretranslate-rs
A LibreTranslate API for Rust.
```
libretranslate = "0.1.6"
```

libretranslate-rs allows you to use open source machine translation in your projects through an easy to use API that connects to the official webpage.

Using it is fairly simple:

```rust
use libretranslate::{translate, Language};

fn main() {
    let input = "Olá Mundo!";
    let language_input = Language::Portuguese;
    let language_output = Language::English;

    let output = match translate(language_input, language_output, input) {
        Ok(output) => output,
        Err(error) => panic!("Translation error: {}", error),
    };

    println!("{}: {}", language_input.pretty(), input);
    println!("{}: {}", language_output.pretty(), output);
}
```

Output:
```
Portuguese: Olá Mundo!
English: Hello world!
```

Written with love, in Rust by Grant Handy.
