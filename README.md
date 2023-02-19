# rust-base64-light

Base64 Encoder, Decoder for Rust.
This is very simple library.

## How to use

Add 'base64_light' to your project's Cargo.toml.

```
[package]
...
[dependencies]
base64_light = "0.1"
```

And use 'base64_encode()' or 'base64_decode_str()' etc ...

```
use base64_light::*;

fn main() {
    let s = "hello!";
    println!("{} => {}", s, base64_encode(s));
}
```

## Manual

- [docs.rs/base64_light](https://docs.rs/base64_light/)
- [repository](https://github.com/kujirahand/rust-base64-light)


