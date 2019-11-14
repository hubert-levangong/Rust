extern crate hex;

use std::fs;
use std::io;
use std::io::Read;
use std::error::Error;

#[derive(Copy, Clone)]
struct Freq {
    code : u8,
    occurences: i32,
}


fn main() -> Result<(), Box<dyn Error>> {
    let filename = String::from("/home/hubert/Documents/Github/Rust/CC/Challenge4/src/4.txt");
    println!("Loading the file");
    let content = file_reader(&filename)?;

    for _line in content.lines() {
        let testvec = hex::decode(_line).expect("Failed to decode the data.");
        let topcode = get_most_frequent_code(&testvec);
        let mut xorvec = Vec::with_capacity(testvec.len());
        for _element in testvec.iter() {
            xorvec.push(topcode);
        }
        let result = fixed_xor(testvec, xorvec);
        match String::from_utf8(result) {
            Ok(o) => {
                if line_is_readable(&o) {println!("Sentence: {}", o);} else {}
            },
            Err(_e) => {/*println!("Invalid line")*/},
        }
    }

    Ok(())
}


fn file_reader(filename : &String) -> Result<String, io::Error> {
    let mut fp = fs::File::open(filename)?;
    let mut s = String::new();

    fp.read_to_string(&mut s)?;

    Ok(s)
}


fn line_is_readable(datain : &String) -> bool {
    let data = datain.as_bytes();
    for &item in data.iter() {
        if (item < 32 || item > 127) && (item != 0) {return false;}
    }
    return true;
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
    //println!("top code: {} - occurences: {}", top.code, top.occurences);
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
        panic!("Lengths differ.")
    }
    for (i, &_item) in buf1.iter().enumerate() {
        res.push(buf1[i] ^ buf2[i]);
    }
    return res;
}