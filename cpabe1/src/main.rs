extern crate crypto;

use rabe::schemes::ac17::*;
use std::time::Instant;
use rand::{Rng, RngCore};
use crypto::aes_gcm::AesGcm;
use crypto::aes::KeySize::KeySize256;
use crypto::aead::AeadEncryptor;
use rustc_serialize::base64::{self, ToBase64};


fn main() {
    println!("Testing AC17 scheme");

    // Dummy data
    let mut data: Vec<u8> = Vec::with_capacity(1000000);
    fill_dummy_data(&mut data);

    let(pk, msk) = setup();
    let policy = String::from(r#"{"AND": [{"ATT": "A"}, {"ATT": "B"}]}"#);

    let mut ts_in = Instant::now();
    let ciphertext: Ac17CpCiphertext = cp_encrypt(&pk, &policy, &data).unwrap();
    let mut ts_out = Instant::now();
    println!("Encryption duration: {:?}", ts_out.duration_since(ts_in));

    let sk: Ac17CpSecretKey = cp_keygen(&msk, &vec!["A".to_string(), "B".to_string(), "C".to_string(), ]).unwrap();

    ts_in = Instant::now();
    let plaintext = cp_decrypt(&sk, &ciphertext);
    ts_out = Instant::now();
    println!("Decryption duration: {:?}", ts_out.duration_since(ts_in));

    match plaintext {
        Some(x) => {
            println!("Decryption successful");
            assert_eq!(x, data);
        },
        None => println!("Decryption failed"),
    }
    println!("ABE part Done.");

    println!("Testing AES.");
    let mut gen = rand::thread_rng();

    // Generating key
    let mut aeskey = vec![0; 32];
    gen.fill_bytes(&mut aeskey);

    // Generating Nonce
    let mut aesnonce = vec![0; 12];
    gen.fill_bytes(&mut aesnonce);

    // Generating AAD
    let mut aesaad = vec![0; 32];
    gen.fill_bytes(&mut aesaad);

    // Display
    println!("AES key length: {}, Capacity: {}", aeskey.len(), aeskey.capacity());
    println!("AES Key: {}", aeskey.to_base64(base64::STANDARD));
    println!("AES nonce: {}", aesnonce.to_base64(base64::STANDARD));
    println!("AES AAD: {}", aesaad.to_base64(base64::STANDARD));

    ts_in = Instant::now();
    let mut cipher = AesGcm::new(KeySize256, &aeskey, &aesnonce, &aesaad);
    let mut ciphertext2 = vec![0; 1000000];
    let mut tag= vec![0; 32];
    cipher.encrypt(&data, &mut ciphertext2, &mut tag);
    ts_out = Instant::now();
    println!("AES encryption duration: {:?}", ts_out.duration_since(ts_in));
}

fn fill_dummy_data(bag: &mut Vec<u8>) {
    let mut rng = rand::thread_rng();
    let mut index = 0;

    while index < 1000000 {
        bag.push(rng.gen());
        index += 1;
    }
}