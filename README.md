# libretranslate-rs
A LibreTranslate API for Rust.
```
libretranslate = "0.1.9"
```

libretranslate-rs allows you to use open source machine translation in your projects through an easy to use API that connects to the official [webpage](https://libretranslate.com/).

## Example
Using it is fairly simple:
```rust
use libretranslate::{Translator, Language};

fn main() {
    let source = Language::Portuguese;
    let target = Language::English;
    let input = "Olá Mundo!";
    let output = Translator::translate(source, target, input).unwrap().output;

    println!("Input {}: {}", source.pretty(), input);
    println!("Output {}: {}", target.pretty(), output);
}
```

Output:
```
Input Portuguese: Olá Mundo!
Output English: Hello world!
```

## Available Languages
- English
- Arabic
- Chinese
- French
- German
- Italian
- Japanese
- Portuguese
- Russian
- Spanish

Written in Rust, with love by Grant Handy.
