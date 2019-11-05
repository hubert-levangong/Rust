extern crate hex;

#[derive(Copy, Clone)]
struct Freq {
    code : u8,
    occurences: i32,
}

fn main() {
    let testvec = hex::decode("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736").expect("Failed to decode hex string");
    println!("data has {} characters.", testvec.len());
    let topcode = get_most_frequent_code(&testvec);
    let mut xorvec = Vec::with_capacity(testvec.len());
    for _element in testvec.iter() {
        //
        xorvec.push(topcode);
    }
    let result = fixed_xor(testvec, xorvec);
    println!("{:?}", result);
    let output  = String::from_utf8(result).expect("Error");
    println!("End result: {}", output);
}


fn get_most_frequent_code(data : &Vec<u8>) -> u8 {
    let mut liste : Vec<Freq> = Vec::new();

    // Calculate frequencies
    for &item in data.iter() {
        let index = in_liste(item, &liste);
        if index != -1 {
            liste[index as usize].occurences += 1;
        } else {
            let elem : Freq = Freq {code: item, occurences: 1};
            liste.push(elem);
        }
    }

    // going through list of occurences to find the top one
    if liste.len() == 0 { return 0};
    let mut top = liste[0];
    for &item in liste.iter() {
        if item.occurences > top.occurences {
            top = item;
        }
    }
    return top.code;
}


fn in_liste(code : u8, liste : &Vec<Freq>) -> i32 {
    for (i, elem) in liste.iter().enumerate() {
        if code == elem.code { return i as i32}
    }
    return -1;
}

fn fixed_xor(buf1 : Vec<u8>, buf2 : Vec<u8>) -> Vec<u8> {
    let mut res = Vec::new();
    if buf1.len() != buf2.len() {
        panic!("Lenghts differ.")
    }
    for (i, &_item) in buf1.iter().enumerate() {
        res.push(buf1[i] ^ buf2[i]);
    }
    return res;
}