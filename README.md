# base64_light

Base64 Encoder, Decoder for Rust.
This is very simple Base64 library.

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
    println!("{} => {}", s, base64_encode(s)); // hello! => aGVsbG8h
    let b = "aGVsbG8h";
    println!("{} <= {}", b, base64_decode_str(b)); // aGVsbG8h <= hello!
}
```

## Methods

### Encode

- base64_encode(in_str: &str) -> String
- base64_encode_bytes(in_bytes: &\[u8\]) -> String
- base64url_encode(in_str: &str) -> String
- base64url_encode_bytes(bytes: &\[u8\]) -> String
- base64_encode_splitlines(in_str: &str) -> String
- base64_encode_splitlines_bytes(in_bytes: &[u8])


### Decode

- pub fn base64_decode(b64str: &str) -> Vec\<u8\> 
- pub fn base64_decode_str(b64str: &str) -> String 

## Link

- [Repository](https://github.com/kujirahand/rust-base64-light)
- [crates.io/crates/base64_light](https://crates.io/crates/base64_light)
- [docs.rs/base64_light](https://docs.rs/base64_light/)

