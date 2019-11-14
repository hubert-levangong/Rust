use std::fs;
use std::io;
use std::io::Read;
use std::error::Error;

//fn main() {
//    let inputstring = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
//    let test = Vec::from(inputstring);
//    let clef= Vec::from("ICE");
//    let mut res : Vec<u8> = Vec::new();
//    encrypt_data_with_code(&test, clef, &mut res);
//    let printout = hex::encode(res);
//    println!("Result: {:?}", printout);
//}

fn main() -> Result<(), Box<dyn Error>> {
//    let filename = String::from("passwd.txt");
//    let inputstring = file_reader(&filename)?;
    let inputstring = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
    let test = Vec::from(inputstring);
    let clef= Vec::from("ICE");
    let mut res : Vec<u8> = Vec::new();
    encrypt_data_with_code(&test, clef, &mut res);
    let printout = hex::encode(res);
    println!("Result: {:?}", printout);

    Ok(())
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


fn file_reader(filename : &String) -> Result<String, io::Error> {
    let mut fp = fs::File::open(filename)?;
    let mut s = String::new();

    fp.read_to_string(&mut s)?;

    Ok(s)
}