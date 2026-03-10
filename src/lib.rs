//! Base64 encoder and decoder.
//!
//! A very simple Base64 library.
//! 
//!
//! # Usage
//! ```
//! use base64_light::*;
//! 
//! let raw = "hello!";
//! let enc = "aGVsbG8h";
//! assert_eq!(base64_encode(raw), enc);
//! assert_eq!(base64_decode_str(enc), raw);
//! ```
//!
//! # Links
//! - [Repository](https://github.com/kujirahand/rust-base64-light)
//! - [crates.io/crates/base64_light](https://crates.io/crates/base64_light)
//! - [docs.rs/base64_light](https://docs.rs/base64_light/)

// Base64 encoding table.
const BASE64TABLE: [char; 64] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J',
    'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T',
    'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd',
    'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n',
    'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x',
    'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7',
    '8', '9', '+', '/'];
// "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

// Base64 decode lookup table.
const BASE64TABLE_DEC: [u8; 256] = [
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x3e,0x00,0x3e,0x00,0x3f,
    0x34,0x35,0x36,0x37,0x38,0x39,0x3a,0x3b,0x3c,0x3d,0x00,0x00,0x00,0x00,0x00,0x00,
    0x00,0x00,0x01,0x02,0x03,0x04,0x05,0x06,0x07,0x08,0x09,0x0a,0x0b,0x0c,0x0d,0x0e,
    0x0f,0x10,0x11,0x12,0x13,0x14,0x15,0x16,0x17,0x18,0x19,0x00,0x00,0x00,0x00,0x3f,
    0x00,0x1a,0x1b,0x1c,0x1d,0x1e,0x1f,0x20,0x21,0x22,0x23,0x24,0x25,0x26,0x27,0x28,
    0x29,0x2a,0x2b,0x2c,0x2d,0x2e,0x2f,0x30,0x31,0x32,0x33,0x00,0x00,0x00,0x00,0x00,
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00];

/// Encodes bytes to Base64.
pub fn base64_encode_bytes(in_bytes: &[u8]) -> String {
    let mut result = String::new();
    // Process 3 bytes (24 bits) as 4 Base64 characters (6 bits each).
    let cnt = in_bytes.len() / 3;
    for i in 0..cnt {
        let n = i * 3;
        // Read 3 input bytes.
        let b24 = ((in_bytes[n+0] as usize) << 16) +
                  ((in_bytes[n+1] as usize) <<  8) +
                  ((in_bytes[n+2] as usize) <<  0);
        // Write 4 output characters.
        result.push(BASE64TABLE[(b24 >> 18) & 0x3f]);
        result.push(BASE64TABLE[(b24 >> 12) & 0x3f]);
        result.push(BASE64TABLE[(b24 >>  6) & 0x3f]);
        result.push(BASE64TABLE[(b24 >>  0) & 0x3f]);
    }
    // Handle remaining bytes when the length is not divisible by 3.
    let mod_val = in_bytes.len() % 3;
    if mod_val == 1 {
        let b24 = (in_bytes[cnt*3] as usize) << 16;
        result.push(BASE64TABLE[(b24 >> 18) & 0x3f]);
        result.push(BASE64TABLE[(b24 >> 12) & 0x3f]);
        result.push_str("==");
    }
    else if mod_val == 2 {
        let b24 = ((in_bytes[cnt*3+0] as usize) << 16) +
            ((in_bytes[cnt*3+1] as usize) << 8);
        result.push(BASE64TABLE[(b24 >> 18) & 0x3f]);
        result.push(BASE64TABLE[(b24 >> 12) & 0x3f]);
        result.push(BASE64TABLE[(b24 >>  6) & 0x3f]);
        result.push('=');
    }
    result
}

/// Encodes a string to Base64.
pub fn base64_encode(in_str: &str) -> String {
    // Convert &str to &[u8].
    let in_bytes = in_str.as_bytes();
    base64_encode_bytes(in_bytes)
}

/// Encodes a string to Base64URL.
pub fn base64url_encode(in_str: &str) -> String {
    // '+' -> '-'
    // '/' -> '_'
    let in_bytes = in_str.as_bytes();
    base64_encode_bytes(in_bytes).replace('+', "-").replace('/', "_")
}

/// Encodes bytes to Base64URL.
pub fn base64url_encode_bytes(bytes: &[u8]) -> String {
    // '+' -> '-'
    // '/' -> '_'
    base64_encode_bytes(bytes).replace('+', "-").replace('/', "_")
}

/// Decodes Base64 into `Vec<u8>`.
pub fn base64_decode(b64str: &str) -> Vec<u8> {
    let mut result: Vec<u8> = vec![];
    // Use decode table.
    let table = BASE64TABLE_DEC;
    // Ignore CR/LF characters.
    let b64 = String::from(b64str).replace("\r", "").replace("\n", "");
    // Convert Base64 text to bytes.
    let b64bytes = b64.as_bytes();
    // Process 4 Base64 characters (24 bits) at a time.
    let cnt = b64bytes.len() / 4;
    for i in 0..cnt {
        // Read 4 input characters (6 bits each = 24 bits total).
        let i0 = b64bytes[i*4+0];
        let i1 = b64bytes[i*4+1];
        let i2 = b64bytes[i*4+2];
        let i3 = b64bytes[i*4+3];
        let c0 = table[i0 as usize] as usize;
        let c1 = table[i1 as usize] as usize;
        let c2 = table[i2 as usize] as usize;
        let c3 = table[i3 as usize] as usize;
        let b24 = (c0 << 18) | (c1 << 12) | (c2 <<  6) | (c3 <<  0);
        // Write 3 output bytes (8 bits each = 24 bits total).
        let b0 = ((b24 >> 16) & 0xFF) as u8;
        let b1 = ((b24 >>  8) & 0xFF) as u8;
        let b2 = ((b24 >>  0) & 0xFF) as u8;
        result.push(b0);
        if i2 as char != '=' { result.push(b1); } // Skip padding at the end.
        if i3 as char != '=' { result.push(b2); } // Skip padding at the end.
    }
    result
}

/// Decodes Base64 into a UTF-8 string (lossy).
pub fn base64_decode_str(b64str: &str) -> String {
    let bytes = base64_decode(b64str);
    String::from_utf8_lossy(&bytes).to_string()
}

/// Encodes a string to Base64 and inserts line breaks every 76 chars (for MIME).
pub fn base64_encode_splitlines(in_str: &str) -> String {
    let in_bytes = in_str.as_bytes();
    base64_encode_splitlines_bytes(in_bytes)
}

/// Encodes bytes to Base64 and inserts line breaks every 76 chars (for MIME).
pub fn base64_encode_splitlines_bytes(in_bytes: &[u8]) -> String {
    let res = base64_encode_bytes(in_bytes);
    let mut lines = String::new();
    for (i, ch) in res.chars().enumerate() {
        lines.push(ch);
        if i % 76 == 75 {
            lines.push_str("\r\n");
        }
    }
    lines.trim_end().to_string()
}

/// For debugging: prints the Base64 decode table.
pub fn base64_table_printer() -> String {
    let mut res = String::new();
    let t = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut table: [u8; 256] = [0; 256];
    for i in 0..256 { table[i] = 0; }
    for (index, c) in t.chars().enumerate() {
        let key = c as usize;
        table[key] = index as u8;
    }
    table['-' as usize] = 62; // '+' in Base64URL
    table['_' as usize] = 63; // '/' in Base64URL
    table['=' as usize] = 0; // Padding
    //
    for (i, v) in table.iter().enumerate() {
        let s = format!("0x{:02x},", v);
        res.push_str(&s);
        if i % 16 == 15 {
            res.push('\n');
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base64_encoder_test() {
        // Basic tests.
        assert_eq!(base64_encode(""), "");
        assert_eq!(base64_encode("f"), "Zg==");
        assert_eq!(base64_encode("fo"), "Zm8=");
        assert_eq!(base64_encode("foo"), "Zm9v");
        assert_eq!(base64_encode("foob"), "Zm9vYg==");
        assert_eq!(base64_encode("fooba"), "Zm9vYmE=");
        assert_eq!(base64_encode("foobar"), "Zm9vYmFy");
        assert_eq!(base64_encode(">>>>>>"), "Pj4+Pj4+");
        assert_eq!(base64_encode("??????"), "Pz8/Pz8/");
        // Additional tests.
        assert_eq!(&base64_encode("HTML"), "SFRNTA==");
        assert_eq!(&base64_encode("hello!"), "aGVsbG8h");
        assert_eq!(&base64_encode("JavaScript"), "SmF2YVNjcmlwdA==");
        assert_eq!(&base64_encode("Rust"), "UnVzdA==");
        assert_eq!(&base64_encode("生姜焼き定食"), "55Sf5aec54S844GN5a6a6aOf"); // UTF-8
        assert_eq!(&base64_encode("★😔"), "4piF8J+YlA==");
        // base64url_encode
        assert_eq!(&base64_encode("🏆☕️💤"), "8J+PhuKYle+4j/CfkqQ=");
        assert_eq!(&base64url_encode("🏆☕️💤"), "8J-PhuKYle-4j_CfkqQ=");
        // base64_encode_splitlines
        assert_eq!(&base64_encode_splitlines("abcdeABCDEabcdeABCDEabcdeABCDEabcdeABCDEabcdeABCDEabcdeABCDE"), "YWJjZGVBQkNERWFiY2RlQUJDREVhYmNkZUFCQ0RFYWJjZGVBQkNERWFiY2RlQUJDREVhYmNkZUFC\r\nQ0RF");
    }

    #[test]
    fn base64_decoder_test() {
        // Basic tests.
        assert_eq!(base64_decode_str(""), "");
        assert_eq!(base64_decode_str("Zg=="), "f");
        assert_eq!(base64_decode_str("Zm8="), "fo");
        assert_eq!(base64_decode_str("Zm9v"), "foo");
        assert_eq!(base64_decode_str("Zm9vYg=="), "foob");
        assert_eq!(base64_decode_str("Zm9vYmE="), "fooba");
        assert_eq!(base64_decode_str("Zm9vYmFy"), "foobar");
        // Additional tests.
        assert_eq!(&base64_decode_str("SFRNTA=="), "HTML");
        assert_eq!(&base64_decode_str("aGVsbG8h"), "hello!");
        assert_eq!(&base64_decode_str("UnVzdA=="), "Rust");
        assert_eq!(&base64_decode_str("SmF2YVNjcmlwdA=="), "JavaScript");
        assert_eq!(&base64_decode_str("55Sf5aec54S844GN5a6a6aOf"), "生姜焼き定食");
        // CR/LF handling
        assert_eq!(&base64_decode_str("aGVsbG8h\r\naGVsbG8h"), "hello!hello!");
        // Base64URL
        assert_eq!(base64_decode_str("4piF8J+YlA=="), "★😔");
        assert_eq!(base64_decode_str("4piF8J-YlA=="), "★😔");
        assert_eq!(base64_decode_str("8J+PhuKYle+4j/CfkqQ="), "🏆☕️💤");
        assert_eq!(base64_decode_str("8J-PhuKYle-4j_CfkqQ="), "🏆☕️💤");
        //
        // Show decode table
        // println!("{}", base64_table_printer());
    }

    #[test]
    fn base64_public_bytes_api_test() {
        let raw = [0xff_u8, 0xef, 0xfa];
        assert_eq!(base64_encode_bytes(&raw), "/+/6");
        assert_eq!(base64url_encode_bytes(&raw), "_-_6");

        let decoded = base64_decode("_-_6");
        assert_eq!(decoded, raw);

        assert_eq!(base64_encode_splitlines_bytes(&[]), "");
    }

    #[test]
    fn base64_splitlines_test() {
        let raw = "abcdeABCDEabcdeABCDEabcdeABCDEabcdeABCDEabcdeABCDEabcdeABCDE";
        let enc = "YWJjZGVBQkNERWFiY2RlQUJDREVhYmNkZUFCQ0RFYWJjZGVBQkNERWFiY2RlQUJDREVhYmNkZUFC\r\nQ0RF";
        assert_eq!(base64_encode_splitlines(raw), enc);
    }

}
