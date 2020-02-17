use std::fs;
use std::io::{Read, Write};
use std::error::Error;
use base64::{decode, decode_config};

//
// Additional testing to confirm
// the complete chain from loading a Base64'ed binary file
// all the way to decoding it using a multiple bytes key
//

fn main() {
    let filename = String::from("/Users/hubert/Documents/Github/Rust/CC/Challenge6/src/5-decoded.txt");
    let mut fp = fs::File::open(filename).expect("Could not open the file");
    let mut hexstring = String::new();
    let nbread = fp.read_to_string(&mut hexstring).expect("Could not load the file");
    println!("Read: {}", hexstring);
    let rawdata = hex::decode(hexstring).expect("Failed to decode string.");
    println!("raw data: {:?}", rawdata);
    let fileout = String::from("/Users/hubert/Documents/Github/Rust/CC/Challenge6/src/5-bin.txt");
    let mut fp2 = fs::File::create(fileout).expect("Could not open the file");
    let res = fp2.write(&rawdata).expect("Failed to write to file");
    fp2.flush();
    println!("Binary file from pretty-printed hex string generated and saved.");

    let clef= Vec::from("ICE");
    let mut res : Vec<u8> = Vec::new();
    encrypt_data_with_code(&rawdata, clef, &mut res);
    let result = String::from_utf8(res).expect("Could not turn result into a String");
    println!("result:\n{}", result);

    println!("\nDecoding Base64'ed encrypted file...");
    let filename2 = String::from("/Users/hubert/Documents/Github/Rust/CC/Challenge6/src/5-binB64.txt");
    let mut fp2 = fs::File::open(filename2).expect("Could not open Base64'ed binary file.");
    let mut b64inp = String::new();
    let nbread2 = fp2.read_to_string(&mut b64inp).expect("Failed to read the Base64'ed binary file.");
    let mut b64inpfixed = b64inp.replace("\n", "");
    let rawdata2 = decode_config(&b64inpfixed, base64::STANDARD).expect("Error during Base64 decoding");
    let clef2= Vec::from("ICE");
    let mut res2 : Vec<u8> = Vec::new();
    encrypt_data_with_code(&rawdata2, clef2, &mut res2);
    let result2 = String::from_utf8(res2).expect("Could not turn result into a String");
    println!("result:\n{}", result2);

}


// Encrypt data using a multi-bytes key
fn encrypt_data_with_code(input : &Vec<u8>, code : Vec<u8>, output : &mut Vec<u8>) -> bool {
    if output.len() != 0 {
        return false;
    }
    let keylength = code.len();

    for (i, &item) in input.iter().enumerate() {
        let element = item ^ code[i % keylength];
        output.push(element);
    }
    return true;
}