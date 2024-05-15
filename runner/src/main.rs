use crypto_lib::{
    arrays::{b64_decode, b64_encode, fixed_xor, hex_decode, hex_encode, xor_repeating},
    crypto::{break_repeating_xor, break_single_xor, decrypt_aes_cbc, decrypt_aes_ecb, pkcs7_pad},
    english::english_score,
};
use std::collections::HashMap;
use std::fs::read_to_string;
use std::str;

fn challenge_1() {
    let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let decoded = hex_decode(input).unwrap();
    println!("#1: Decoded: {}", str::from_utf8(&decoded).unwrap());
    let encoded = b64_encode(&decoded);
    println!("Encoded: {}", &encoded);
}

fn challenge_2() {
    let a = "1c0111001f010100061a024b53535009181c";
    let b = "686974207468652062756c6c277320657965";
    let a_bytes = hex_decode(a).unwrap();
    let b_bytes = hex_decode(b).unwrap();
    let c = fixed_xor(&a_bytes, &b_bytes);
    let result = hex_encode(&c);
    println!("source1: {}", str::from_utf8(&a_bytes).unwrap());
    println!("source2: {}", str::from_utf8(&b_bytes).unwrap());
    println!("result : {}", str::from_utf8(&c).unwrap());
    println!("#2: {}", result);
}

fn challenge_3() {
    let decoded =
        hex_decode("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736").unwrap();
    let result = break_single_xor(&decoded);
    println!("#3: {}", result);
}

fn challenge_4() {
    let mut max_score = 0.0;
    let mut result = String::new();
    for line in read_to_string("4.txt").unwrap().lines() {
        let bytes = hex_decode(line).unwrap();
        let plaintext = break_single_xor(&bytes);
        let score = english_score(&plaintext);
        if score > max_score {
            max_score = score;
            result = plaintext;
        }
    }
    println!("#4: {:?}", result);
}

fn challenge_5() {
    let plaintext = b"Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
    let key = b"ICE";
    let ciphertext = xor_repeating(plaintext, key);
    let result = hex_encode(&ciphertext);
    println!("#5: {:?}", result);
}

fn challenge_6() {
    let ciphertext = read_to_string("6.txt").unwrap();
    let ciphertext = ciphertext.replace("\n", "");
    let ciphertext = b64_decode(&ciphertext).unwrap();
    let result = break_repeating_xor(&ciphertext);
    println!("#6: {}...", &str::from_utf8(&result).unwrap()[0..80]);
}

fn challenge_7() {
    let key = b"YELLOW SUBMARINE";
    let ciphertext = read_to_string("7.txt").unwrap();
    let ciphertext = ciphertext.replace("\n", "");
    let ciphertext = b64_decode(&ciphertext).unwrap();
    let plaintext = decrypt_aes_ecb(&ciphertext, key, true);
    println!(
        "#7: {}...",
        &str::from_utf8(&plaintext.unwrap()).unwrap()[0..80]
    );
}

fn challenge_8() {
    let mut max_repeats = 0;
    let mut result = String::new();
    for line in read_to_string("8.txt").unwrap().lines() {
        let bytes = hex_decode(line).unwrap();
        let chunks = bytes.chunks(16);
        let mut duplicates = HashMap::new();
        for chunk in chunks {
            let count = duplicates.entry(chunk).or_insert(0);
            *count += 1;
        }
        let repeats = duplicates.values().max().unwrap();
        if *repeats > max_repeats {
            max_repeats = *repeats;
            result = line.to_string();
        }
    }
    println!("#8: {:?}", result);
}

fn challenge_9() {
    let input = b"YELLOW SUBMARINE";
    let expected = b"YELLOW SUBMARINE\x04\x04\x04\x04";
    let result = pkcs7_pad(input, 20);
    assert_eq!(result, expected);
    println!("#9: {:?}", str::from_utf8(&result).unwrap());
}

fn challenge_10() {
    let ciphertext = read_to_string("7.txt").unwrap();
    let ciphertext = ciphertext.replace("\n", "");
    let ciphertext = b64_decode(&ciphertext).unwrap();
    let plaintext =
        decrypt_aes_cbc(&ciphertext, b"YELLOW SUBMARINE", vec![0; 16].as_slice()).unwrap();
    println!("#10: {}...", &str::from_utf8(&plaintext).unwrap()[0..80]);
}

fn main() {
    challenge_1();
    challenge_2();
    challenge_3();
    challenge_4();
    challenge_5();
    challenge_6();
    challenge_7();
    challenge_8();
    challenge_9();
    challenge_10();
}
