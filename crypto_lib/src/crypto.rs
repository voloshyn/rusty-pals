use crate::arrays::{fixed_xor, hamming_distance, single_xor, xor_repeating};
use crate::english::english_score;
extern crate openssl;
use openssl::symm::{Cipher, Crypter, Mode};
use std::str;

pub fn break_single_xor(buf: &[u8]) -> String {
    let mut max_score = 0.0;
    let mut result = String::new();
    for i in 0..255 {
        let decoded = single_xor(buf, i);
        let result_str = str::from_utf8(&decoded);
        let (score, text) = match result_str {
            Ok(res) => (english_score(res), res),
            Err(_) => (0.0, ""),
        };
        if score > max_score {
            max_score = score;
            result = text.to_string();
        }
    }
    result
}

pub fn guess_key_size(ciphertext: &[u8]) -> usize {
    let mut minimal_distance = u32::MAX;
    let mut key_size = 0;
    for size in 2..=40 {
        let chunks = ciphertext.len() / size;
        let mut distances: Vec<u32> = vec![];
        for i in 0..chunks - 1 {
            let a = &ciphertext[i * size..(i + 1) * size];
            let b = &ciphertext[(i + 1) * size..(i + 2) * size];
            let distance = hamming_distance(a, b);
            distances.push(distance);
        }
        let average_distance: u32 =
            distances.iter().sum::<u32>() / distances.len() as u32 / size as u32;
        if average_distance < minimal_distance {
            minimal_distance = average_distance;
            key_size = size;
        }
    }
    key_size
}

pub fn break_repeating_xor(ciphertext: &[u8]) -> Vec<u8> {
    let key_size = guess_key_size(ciphertext);
    println!("Key size: {}", key_size);
    let mut key = vec![];
    for i in 0..key_size {
        let block: Vec<u8> = ciphertext
            .iter()
            .skip(i)
            .step_by(key_size)
            .cloned()
            .collect();
        let mut block_bytes = vec![];
        for byte in 0..=255 {
            let key_byte = vec![byte];
            let plaintext = xor_repeating(&block, &key_byte);
            let text = match str::from_utf8(&plaintext) {
                Ok(text) => text,
                Err(_) => continue,
            };
            let score = english_score(text);
            block_bytes.push((score, byte));
        }
        block_bytes.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
        key.push(block_bytes[0].1);
    }
    println!("Key: {:?}", str::from_utf8(key.as_slice()).unwrap());
    xor_repeating(ciphertext, &key)
}

pub fn pkcs7_pad(input: &[u8], block_size: usize) -> Vec<u8> {
    let mut padding = block_size - input.len() % block_size;
    if padding == 0 {
        padding = block_size;
    }
    let mut result = input.to_vec();
    for _ in 0..padding {
        result.push(padding as u8);
    }
    result
}

pub fn pkcs7_unpad(input: &[u8]) -> Vec<u8> {
    let padding = input[input.len() - 1] as usize;
    let mut result = input.to_vec();
    result.truncate(input.len() - padding);
    result
}

pub fn decrypt_aes_ecb(
    ciphertext: &[u8],
    key: &[u8],
    pad: bool,
) -> Result<Vec<u8>, openssl::error::ErrorStack> {
    assert_eq!(key.len(), 16);
    let mut decrypter = Crypter::new(Cipher::aes_128_ecb(), Mode::Decrypt, key, None).unwrap();
    decrypter.pad(pad);
    let mut decrypted = vec![0; ciphertext.len() + 16];
    let mut size = decrypter.update(ciphertext, &mut decrypted).unwrap();
    size += decrypter.finalize(&mut decrypted).unwrap();
    if !pad {
        decrypted.truncate(ciphertext.len());
    } else {
        decrypted.truncate(size);
    }
    Ok(decrypted)
}

pub fn encrypt_aes_ecb(
    plaintext: &[u8],
    key: &[u8],
    pad: bool,
) -> Result<Vec<u8>, openssl::error::ErrorStack> {
    assert_eq!(key.len(), 16);
    let mut crypter = Crypter::new(Cipher::aes_128_ecb(), Mode::Encrypt, key, None).unwrap();
    crypter.pad(pad);
    let mut cipherblock = vec![0; plaintext.len() + 16];
    let mut size = crypter.update(plaintext, &mut cipherblock).unwrap();
    size += crypter.finalize(&mut cipherblock).unwrap();
    if !pad {
        cipherblock.truncate(plaintext.len());
    } else {
        cipherblock.truncate(size);
    }
    Ok(cipherblock)
}

pub fn encrypt_aes_cbc(
    plaintext: &[u8],
    key: &[u8],
    iv: &[u8],
) -> Result<Vec<u8>, openssl::error::ErrorStack> {
    let plaintext = pkcs7_pad(plaintext, 16);
    let result = plaintext
        .chunks(16)
        .fold((iv.to_vec(), Vec::new()), |(iv, mut acc), block| {
            let xored = fixed_xor(block, &iv);
            let encrypted = encrypt_aes_ecb(&xored, key, false).unwrap();
            acc.extend_from_slice(&encrypted);
            (iv.to_vec(), acc)
        });
    Ok(result.1)
}

pub fn decrypt_aes_cbc(
    ciphertext: &[u8],
    key: &[u8],
    iv: &[u8],
) -> Result<Vec<u8>, openssl::error::ErrorStack> {
    let result = ciphertext
        .chunks(16)
        .fold((iv.to_vec(), Vec::new()), |(iv, mut acc), block| {
            let decrypted = decrypt_aes_ecb(block, key, false).unwrap();
            let xored = fixed_xor(&decrypted, &iv);
            acc.extend_from_slice(&xored);
            (iv.to_vec(), acc)
        });
    Ok(pkcs7_unpad(result.1.as_slice()))
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_aes_ecb() {
        let key = b"YELLOW SUBMARINE";
        let plaintext = b"Hello, World!";
        let ciphertext = encrypt_aes_ecb(plaintext, key, true).unwrap();
        let decrypted = decrypt_aes_ecb(&ciphertext, key, true).unwrap();
        assert_eq!(plaintext, decrypted.as_slice());
        assert_ne!(ciphertext, plaintext);
    }

    #[test]
    fn test_aes_cbc() {
        let key = b"YELLOW SUBMARINE";
        let iv = vec![0; 16];
        let plaintext = b"Hello, World! Hello world, hello world!";
        let ciphertext = encrypt_aes_cbc(plaintext, key, &iv).unwrap();
        let decrypted = decrypt_aes_cbc(&ciphertext, key, &iv).unwrap();
        assert_eq!(plaintext, decrypted.as_slice());
        assert_ne!(ciphertext, plaintext);
    }
}
