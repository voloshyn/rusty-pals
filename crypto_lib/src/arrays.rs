use std::fmt::Write;
use std::num::ParseIntError;
use std::str;

pub fn hex_decode(hex: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i + 2], 16))
        .collect()
}

pub fn hex_encode(buf: &[u8]) -> String {
    buf.iter().fold(String::new(), |mut output, b| {
        let _ = write!(output, "{:02x}", b);
        output
    })
}

pub fn b64_decode(base64: &str) -> Result<Vec<u8>, ParseIntError> {
    let b64_alphabet = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut output = Vec::new();
    let mut accum = 0;
    let mut bits = 0;
    for ch in base64.chars() {
        if ch == '=' {
            break;
        }
        let index = b64_alphabet.iter().position(|&x| x as char == ch).unwrap() as u32;
        accum = (accum << 6) | index;
        bits += 6;
        if bits >= 8 {
            bits -= 8;
            output.push((accum >> bits) as u8);
        }
    }
    Ok(output)
}

pub fn b64_encode(buf: &[u8]) -> String {
    let b64_alphabet = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut output = String::new();
    for chunk in buf.chunks(3) {
        let accum = match chunk.len() {
            3 => (chunk[0] as u32) << 16 | (chunk[1] as u32) << 8 | chunk[2] as u32,
            2 => (chunk[0] as u32) << 16 | (chunk[1] as u32) << 8,
            1 => (chunk[0] as u32) << 16,
            _ => unreachable!(),
        };

        for i in (0..4).rev() {
            if chunk.len() * 8 > (3 - i) * 6 {
                let index = ((accum >> (i * 6)) & 0x3F) as usize;
                let ch = b64_alphabet[index] as char;
                output.push(ch);
            } else {
                output.push('=');
            }
        }
    }
    output
}

pub fn fixed_xor(a: &[u8], b: &[u8]) -> Vec<u8> {
    a.iter().zip(b.iter()).map(|(x, y)| x ^ y).collect()
}

pub fn single_xor(a: &[u8], b: u8) -> Vec<u8> {
    a.iter().map(|x| x ^ b).collect()
}

pub fn xor_repeating(a: &[u8], b: &[u8]) -> Vec<u8> {
    a.iter().zip(b.iter().cycle()).map(|(x, y)| x ^ y).collect()
}

pub fn hamming_distance(a: &[u8], b: &[u8]) -> u32 {
    assert_eq!(a.len(), b.len());
    a.iter()
        .zip(b.iter())
        .map(|(x, y)| (x ^ y).count_ones())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_to_bytes() {
        let hex = "48656c6c6f";
        let expected_bytes = b"Hello".to_vec();
        let result = hex_decode(hex).unwrap();
        assert_eq!(result, expected_bytes);
    }

    #[test]
    fn test_bytes_to_base64() {
        let bytes = b"Hello";
        let expected_base64 = "SGVsbG8=";
        let result = b64_encode(bytes);
        assert_eq!(result, expected_base64);
    }

    #[test]
    fn test_xor_single() {
        let a = b"Hello";
        let b = 0xA;
        let expected_result = vec![0x42, 0x6F, 0x66, 0x66, 0x65];
        let result = single_xor(a, b);
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_xor_repeating() {
        let a = b"Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
        let b = b"ICE";
        let expected_result = hex_decode("0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f").unwrap();
        let result = xor_repeating(a, b);
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_hamming_distance() {
        let a = b"this is a test";
        let b = b"wokka wokka!!!";
        let expected_result = 37;
        let result = hamming_distance(a, b);
        assert_eq!(result, expected_result);
    }
}
