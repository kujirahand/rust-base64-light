# base64_light

A simple Base64 encoder and decoder library for Rust.

## Usage

Add `base64_light` to your project:

```bash
cargo add base64_light
```

### Example

Here is a simple example to help you get started.

```rs
use base64_light::*;

fn main() {
    // Encode string
    let s = "hello!";
    println!("{} → {}", s, base64_encode(s)); // hello! → aGVsbG8h

    // Decode string
    let b64str = "aGVsbG8h";
    println!("{} → {}", b64str, base64_decode_str(b64str)); // aGVsbG8h → hello!

    // Encode file
    let filename = "test.png";
    let file_content = std::fs::read(filename).unwrap();
    let encoded = base64_encode_bytes(&file_content);
    println!("{}", encoded);
}
```

## Methods

### Encoding

- `base64_encode(in_str: &str) -> String`
- `base64_encode_bytes(in_bytes: &[u8]) -> String`
- `base64url_encode(in_str: &str) -> String`
- `base64url_encode_bytes(bytes: &[u8]) -> String`
- `base64_encode_splitlines(in_str: &str) -> String`
- `base64_encode_splitlines_bytes(in_bytes: &[u8]) -> String`

### Decoding

- `base64_decode(b64str: &str) -> Vec<u8>`
- `base64_decode_str(b64str: &str) -> String`

## Links

- [Repository](https://github.com/kujirahand/rust-base64-light)
- [crates.io/crates/base64_light](https://crates.io/crates/base64_light)
- [docs.rs/base64_light](https://docs.rs/base64_light/)
