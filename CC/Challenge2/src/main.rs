extern crate hex;

fn main() {
    let set1 = hex::decode("1c0111001f010100061a024b53535009181c").expect("Decoding failed");
    let set2 = hex::decode("686974207468652062756c6c277320657965").expect("Decoding failed");

    let res = fixed_xor(set1, set2);
    println!("Fixed XOR result: {}", hex::encode(res));
}

fn fixed_xor(buf1 : Vec<u8>, buf2 : Vec<u8>) -> Vec<u8> {
    let mut res = Vec::new();
    if buf1.len() != buf2.len() {
        panic!("Lenghts differ.")
    }
    for (i, &item) in buf1.iter().enumerate() {
        res.push(buf1[i] ^ buf2[i]);
    }
    return res;
}