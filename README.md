# libretranslate-rs
A LibreTranslate API for Rust.
```
libretranslate = "0.1.6"
```

libretranslate-rs allows you to use open source machine translation in your projects through an easy to use API that connects to the official webpage.

Using it is fairly simple:

```rust
use libretranslate::{Translator, Language};

fn main() {
    let input = "Olá Mundo!";
    let source = Language::Portuguese;
    let target = Language::English;

    match Translator::translate(source, target, input) {
        Ok(data) => println!("{}: {}\n{}: {}", data.source.pretty(), data.input, data.target.pretty(), data.output),
        Err(error) => panic!("{}", error),
    };
}
```

Output:
```
Portuguese: Olá Mundo!
English: Hello world!
```

Written with love, in Rust by Grant Handy.
