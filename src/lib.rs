//! Base64 Encoder & Decoder
//!
//! This is very simple Base64 library.
//! 
//! - [docs.rs/base64_light](https://docs.rs/base64_light/)
//! - [repository](https://github.com/kujirahand/rust-base64-light)

/// encode Base64 &str
pub fn base64_encode(in_str: &str) -> String {
    // convert &str to &[u8]
    let in_bytes = in_str.as_bytes();
    base64_encode_bytes(in_bytes)
}

/// encode Base64 bytes
pub fn base64_encode_bytes(in_bytes: &[u8]) -> String {
    // table
    let t = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let table = t.chars().collect::<Vec<char>>();
    let mut result = String::new();
    // for 3bytes = 24bit = 6bit * 4
    let cnt = in_bytes.len() / 3;
    for i in 0..cnt {
        let n = i * 3;
        // input 3bytes
        let b24 = ((in_bytes[n+0] as usize) << 16) +
                  ((in_bytes[n+1] as usize) <<  8) +
                  ((in_bytes[n+2] as usize) <<  0);
        // output 4chars
        result.push(table[(b24 >> 18) & 0x3f]);
        result.push(table[(b24 >> 12) & 0x3f]);
        result.push(table[(b24 >>  6) & 0x3f]);
        result.push(table[(b24 >>  0) & 0x3f]);
    }
    // Handles undivisible bytes
    let mod_val = in_bytes.len() % 3;
    if mod_val == 1 {
        let b24 = (in_bytes[cnt*3] as usize) << 16;
        result.push(table[(b24 >> 18) & 0x3f]);
        result.push(table[(b24 >> 12) & 0x3f]);
        result.push_str("==");
    }
    else if mod_val == 2 {
        let b24 = ((in_bytes[cnt*3+0] as usize) << 16) +
            ((in_bytes[cnt*3+1] as usize) << 8);
        result.push(table[(b24 >> 18) & 0x3f]);
        result.push(table[(b24 >> 12) & 0x3f]);
        result.push(table[(b24 >>  6) & 0x3f]);
        result.push('=');
    }
    result
}

/// encode Base64URL &str
pub fn base64url_encode(in_str: &str) -> String {
    // '+' -> '-'
    // '/' -> '_'
    let in_bytes = in_str.as_bytes();
    base64_encode_bytes(in_bytes).replace('+', "-").replace('/', "_")
}

/// encode Base64URL bytes
pub fn base64url_encode_bytes(bytes: &[u8]) -> String {
    // '+' -> '-'
    // '/' -> '_'
    base64_encode_bytes(bytes).replace('+', "-").replace('/', "_")
}

/// Decode Base64 to Vec<u8>
pub fn base64_decode(b64str: &str) -> Vec<u8> {
    let mut result: Vec<u8> = vec![];
    // make table
    let t = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut table: [u8; 256] = [0; 256];
    for i in 0..256 { table[i] = 0; }
    for (index, c) in t.chars().enumerate() {
        let key = c as usize;
        table[key] = index as u8;
    }
    table['-' as usize] = 62; // '+' for base64url
    table['_' as usize] = 63; // '/' for base64url
    table['=' as usize] = 0; // padding
    // replace (CR|LF)
    let b64 = String::from(b64str).replace("\r", "").replace("\n", "");
    // b64 to [u8]
    let b64bytes = b64.as_bytes();
    // 24bit
    let cnt = b64bytes.len() / 4;
    for i in 0..cnt {
        // input 4char * 6bit = 24bit
        let i0 = b64bytes[i*4+0];
        let i1 = b64bytes[i*4+1];
        let i2 = b64bytes[i*4+2];
        let i3 = b64bytes[i*4+3];
        let c0 = table[i0 as usize] as usize;
        let c1 = table[i1 as usize] as usize;
        let c2 = table[i2 as usize] as usize;
        let c3 = table[i3 as usize] as usize;
        let b24 = (c0 << 18) | (c1 << 12) | (c2 <<  6) | (c3 <<  0);
        // output 3char * 8bit = 24bit
        let b0 = ((b24 >> 16) & 0xFF) as u8;
        let b1 = ((b24 >>  8) & 0xFF) as u8;
        let b2 = ((b24 >>  0) & 0xFF) as u8;
        result.push(b0);
        if i2 as char != '=' { result.push(b1); } // check last
        if i3 as char != '=' { result.push(b2); } // check last
    }
    result
}

/// Decode Base64 to String
pub fn base64_decode_str(b64str: &str) -> String {
    let bytes = base64_decode(b64str);
    String::from_utf8_lossy(&bytes).to_string()
}

/// Base64 Encode &str, and split line 76 chars (for MIME)
pub fn base64_encode_splitlines_str(in_str: &str) -> String {
    let in_bytes = in_str.as_bytes();
    base64_encode_splitlines_bytes(in_bytes)
}

/// Base64 Encode bytes, and split line 76 chars (for MIME)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base64_encoder_test() {
        assert_eq!(base64_encode(""), "");
        assert_eq!(base64_encode("f"), "Zg==");
        assert_eq!(base64_encode("fo"), "Zm8=");
        assert_eq!(base64_encode("foo"), "Zm9v");
        assert_eq!(base64_encode("foob"), "Zm9vYg==");
        assert_eq!(base64_encode("fooba"), "Zm9vYmE=");
        assert_eq!(base64_encode("foobar"), "Zm9vYmFy");
        //
        assert_eq!(&base64_encode("HTML"), "SFRNTA==");
        assert_eq!(&base64_encode("hello!"), "aGVsbG8h");
        assert_eq!(&base64_encode("JavaScript"), "SmF2YVNjcmlwdA==");
        assert_eq!(&base64_encode("生姜焼き定食"), "55Sf5aec54S844GN5a6a6aOf");
        assert_eq!(&base64_encode("★😔"), "4piF8J+YlA==");
        assert_eq!(&base64_encode("🏆☕️💤"), "8J+PhuKYle+4j/CfkqQ=");
        //
        assert_eq!(&base64url_encode("🏆☕️💤"), "8J-PhuKYle-4j_CfkqQ=");
        //
        assert_eq!(&base64_encode_splitlines_str("abcdeABCDEabcdeABCDEabcdeABCDEabcdeABCDEabcdeABCDEabcdeABCDE"), "YWJjZGVBQkNERWFiY2RlQUJDREVhYmNkZUFCQ0RFYWJjZGVBQkNERWFiY2RlQUJDREVhYmNkZUFC\r\nQ0RF");
    }
    #[test]
    fn base64_decoder_test() {
        assert_eq!(&base64_decode_str("SFRNTA=="), "HTML");
        assert_eq!(&base64_decode_str("aGVsbG8h"), "hello!");
        assert_eq!(&base64_decode_str("SmF2YVNjcmlwdA=="), "JavaScript");
        assert_eq!(&base64_decode_str("55Sf5aec54S844GN5a6a6aOf"), "生姜焼き定食");
        //
        assert_eq!(base64_decode_str(""), "");
        assert_eq!(base64_decode_str("Zg=="), "f");
        assert_eq!(base64_decode_str("Zm8="), "fo");
        assert_eq!(base64_decode_str("Zm9v"), "foo");
        assert_eq!(base64_decode_str("Zm9vYg=="), "foob");
        assert_eq!(base64_decode_str("Zm9vYmE="), "fooba");
        assert_eq!(base64_decode_str("Zm9vYmFy"), "foobar");
        // CR+LF
        assert_eq!(&base64_decode_str("aGVsbG8h\r\naGVsbG8h"), "hello!hello!");
        // Base64url
        assert_eq!(base64_decode_str("4piF8J+YlA=="), "★😔");
        assert_eq!(base64_decode_str("4piF8J-YlA=="), "★😔");
        assert_eq!(base64_decode_str("8J+PhuKYle+4j/CfkqQ="), "🏆☕️💤");
        assert_eq!(base64_decode_str("8J-PhuKYle-4j_CfkqQ="), "🏆☕️💤");
    }
}
