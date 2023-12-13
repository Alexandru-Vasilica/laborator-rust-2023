fn get_encoding(x: u8) -> char {
    match x {
        0..=25 => return ('A' as u8 + x) as char,
        26..=51 => return ('a' as u8 + (x - 26)) as char,
        52..=61 => return ('0' as u8 + (x - 52)) as char,
        62 => return '+',
        63 => return '/',
        _ => return '=',
    }
}
/// Encodes an arry of bytes and outputs its base64 representation as a String
/// # Example:
///
/// ```
/// # use base64::encode;
/// assert_eq!(encode(b"Hello World!"),"SGVsbG8gV29ybGQh");
/// assert_eq!(encode(&[1,2,3,4]),"AQIDBA==");
/// ```
pub fn encode(input: &[u8]) -> String {
    let mut i = 0;
    let mut output = String::new();
    while i < input.len() {
        let mut combined: u32 = (input[i] as u32) << (2 * 8);
        let mut pad: u8 = 0;
        if i + 1 < input.len() {
            combined = combined | ((input[i + 1] as u32) << 8);

            if i + 2 < input.len() {
                combined = combined | (input[i + 2] as u32);
            } else {
                pad = 1;
            }
        } else {
            pad = 2;
        }
        let mut chars: [u8; 4] = [0; 4];
        for j in 0..4 {
            chars[j] = ((combined >> (j * 6)) & 0b111_111) as u8;
            if (j as u8) < pad {
                chars[j] = 64;
            }
        }
        for j in (0..4).rev() {
            output.push(get_encoding(chars[j]));
        }
        i += 3
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn no_padding() {
        assert_eq!(encode(b"light wor"), "bGlnaHQgd29y");
        assert_eq!(encode(b"base64"), "YmFzZTY0");
    }
    #[test]
    fn padding_once() {
        assert_eq!(encode(b"Rust create"), "UnVzdCBjcmVhdGU=");
        assert_eq!(encode(b"laborator 2"), "bGFib3JhdG9yIDI=");
    }
    #[test]
    fn padding_twice() {
        assert_eq!(encode(b"Test"), "VGVzdA==");
        assert_eq!(encode(b"light work"), "bGlnaHQgd29yaw==");
    }
}
