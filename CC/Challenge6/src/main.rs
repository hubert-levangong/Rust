use std::fs;
use std::io::{Read, Write};
use std::error::Error;
use base64::{decode, decode_config};

#[derive(Copy, Clone)]
struct Distance {
    keysz : usize,
    dist: f32,
}

#[derive(Copy, Clone)]
struct Freq {
    code : u8,
    occurences: i32,
}

// From steps 3 & 4
// Measuring over 2 consecutive blocks of key size and averaging out.
// average: [2] = 2.25
// average: [3] = 2.3333335
// average: [4] = 3.125
// average: [5] = 2.3000002
// average: [6] = 3.3333335
// average: [7] = 3.2857141
// average: [8] = 2.9375
// Looks like key size is 2 bytes.

const KEY_SIZE: usize = 2;

fn main() {
    // Steps 1 & 2
    let in1 = Vec::from("this is a test");
    let in2 = Vec::from("wokka wokka!!!");
    let res = hamming_distance(&in1, &in2);
    println!("Hamming distance = {}", res);

    // First we read the data from the file, remove EOL chars and Base64 decode it
    let filename = String::from("/Users/hubert/Documents/Github/Rust/CC/Challenge6/src/6.txt");
    let mut fp = fs::File::open(filename).expect("Could not open the file");
    let mut b64data = String::new();
    let nbread = fp.read_to_string(&mut b64data).expect("Could not read the file");
    let mut b64noEOLdata = b64data.replace("\n", "");
    let rawdata = decode_config(&b64noEOLdata, base64::STANDARD).expect("Could not Base64 decode the data");
    println!("Base64 decoded data is {} bytes long", rawdata.len());

//    //Writing Base64 decoded file to disk to validate decoding
//    let exportfilename = String::from("/Users/hubert/Documents/Github/Rust/CC/Challenge6/src/6-decoded.txt");
//    let mut fp2 = fs::File::create(exportfilename).expect("Could not create the output file.");
//    fp2.write(&rawdata).expect("Could not write to file.");
//    fp2.flush();

//    // Steps 3 & 4
//    let mut dists= Vec::new();
//    keysize_distances(&rawdata,2, 40, &mut dists);
//    println!("result has {} distances", dists.len());
//    for &elem in dists.iter() {
//        println!("[{}:{}]", elem.keysz, elem.dist);
//    }
//
//    // Calculating 2nd batch of distances
//    let mut dists2 = Vec::new();
//    keysize_distances2(&rawdata,2, 40, &mut dists2);
//    println!("result has {} distances", dists2.len());
//    for &elem in dists2.iter() {
//        println!("[{}:{}]", elem.keysz, elem.dist);
//    }
//
//    for i in 0..dists.len() {
//        println!("average: [{}] = {}", dists[i].keysz, (dists[i].dist + dists2[i].dist)/2 as f32);
//    }


    // Steps 5 & 6
    let mut blocks : Vec<Vec<u8>> = Vec::new();
    fill_blocks(&rawdata,&mut blocks);
    println!("Blocks: {:?}", blocks);
    let bl = blocks.len();
    let mut tblocks : Vec<Vec<u8>> = Vec::with_capacity(KEY_SIZE);
    tblocks.resize(KEY_SIZE, Vec::new());
    transpose_blocks(&blocks, &mut tblocks);
//    println!("Transposed Blocks: {:?}", tblocks);

// Steps 7 & 8
    // key size is 2
    let mut inputVector0 = Vec::with_capacity(tblocks[0].len());
    for &item in tblocks[0].iter() {
        inputVector0.push(item);
    }
//    println!("Input vector length: {}", inputVector0.len());
//    println!("Clone of block 0: {:?}", inputVector0);
    let topcode0 = get_most_frequent_code(&inputVector0);
    println!("Most frequent code for bloc 0: {}", topcode0);


    let mut inputVector1 = Vec::with_capacity(tblocks[1].len());
    for &item in tblocks[1].iter() {
        inputVector1.push(item);
    }
    let topcode1 = get_most_frequent_code(&inputVector1);
    println!("Most frequent code for bloc 1: {}", topcode1);


//    let mut inputVector2 = Vec::with_capacity(tblocks[2].len());
//    for &item in tblocks[2].iter() {
//        inputVector2.push(item);
//    }
//    let topcode2 = get_most_frequent_code(&inputVector2);
//    println!("Most frequent code for bloc 2: {}", topcode2);
//
//
//    let mut inputVector3 = Vec::with_capacity(tblocks[3].len());
//    for &item in tblocks[3].iter() {
//        inputVector3.push(item);
//    }
//    let topcode3 = get_most_frequent_code(&inputVector3);
//    println!("Most frequent code for bloc 3: {}", topcode3);
//
//
//    let mut inputVector4 = Vec::with_capacity(tblocks[4].len());
//    for &item in tblocks[4].iter() {
//        inputVector4.push(item);
//    }
//    let topcode4 = get_most_frequent_code(&inputVector4);
//    println!("Most frequent code for bloc 4: {}", topcode4);
//
//
//    let mut inputVector5 = Vec::with_capacity(tblocks[5].len());
//    for &item in tblocks[5].iter() {
//        inputVector5.push(item);
//    }
//    let topcode5 = get_most_frequent_code(&inputVector5);
//    println!("Most frequent code for bloc 5: {}", topcode5);
//
//
//    let mut inputVector6 = Vec::with_capacity(tblocks[6].len());
//    for &item in tblocks[6].iter() {
//        inputVector6.push(item);
//    }
//    let topcode6 = get_most_frequent_code(&inputVector6);
//    println!("Most frequent code for bloc 6: {}", topcode6);
//
//
//    let mut inputVector7 = Vec::with_capacity(tblocks[7].len());
//    for &item in tblocks[7].iter() {
//        inputVector7.push(item);
//    }
//    let topcode7 = get_most_frequent_code(&inputVector7);
//    println!("Most frequent code for bloc 7: {}", topcode7);


    let mut key = Vec::with_capacity(KEY_SIZE);
    key.push(topcode0);
    key.push(topcode1);
//    key.push(topcode2);
//    key.push(topcode3);
//    key.push(topcode4);
//    key.push(topcode5);
//    key.push(topcode6);
//    key.push(topcode7);

    let v1 = key.clone();
    let keystr = String::from_utf8(v1).expect("ok");
    println!("Clef: {}", keystr);

    // Decoding the data
    let mut res : Vec<u8> = Vec::new();
    encrypt_data_with_code(&rawdata, key, &mut res);
    let outputres = String::from_utf8(res).expect("Could not export the res into a String.");
    println!("Result: {}", outputres);
    //let printout = hex::encode(res);
    //println!("Result: {:?}", printout);

//    let mut xorvec = Vec::with_capacity(inputVector.len());
//    for _element in inputVector.iter() {
//        xorvec.push(topcode);
//    }
//    let result = fixed_xor(inputVector, xorvec);
//    println!("{:?}", result);
//    let output  = String::from_utf8(result).expect("Error");
//    println!("End result: {}", output);
}


fn keysize_distances(rawdata : &Vec<u8>, keysizemin : usize, keysizemax : usize, distances : &mut Vec<Distance>) {
    println!("key sizes in ({}, {})", keysizemin, keysizemax);
    for keysize in keysizemin..keysizemax+1 {
        let mut in1 : Vec<u8> = Vec::with_capacity(keysize);
        in1.resize(keysize, 0);
        let mut in2 : Vec<u8> = Vec::with_capacity(keysize);
        in2.resize(keysize, 0);
        fill_two_vectors(rawdata, &mut in1, &mut in2);
        let val : f32 = hamming_distance(&in1, &in2) as f32 / (keysize as f32);
        let dst: Distance = Distance {keysz: keysize, dist: val};
        distances.push(dst);
    }
}

fn keysize_distances2(rawdata : &Vec<u8>, keysizemin : usize, keysizemax : usize, distances : &mut Vec<Distance>) {
    println!("key sizes in ({}, {})", keysizemin, keysizemax);
    for keysize in keysizemin..keysizemax+1 {
        let mut in1 : Vec<u8> = Vec::with_capacity(keysize);
        in1.resize(keysize, 0);
        let mut in2 : Vec<u8> = Vec::with_capacity(keysize);
        in2.resize(keysize, 0);
        fill_two_vectors2(rawdata, &mut in1, &mut in2);
        let val : f32 = hamming_distance(&in1, &in2) as f32 / (keysize as f32);
        let dst: Distance = Distance {keysz: keysize, dist: val};
        distances.push(dst);
    }
}

fn hamming_distance(inp1 : &Vec<u8>, inp2 : &Vec<u8>) -> u32 {
    let mut distance : u32 = 0;
    if inp1.len() != inp2.len() {
        panic!("[Hamming Distance] Different lengths in input.");
    }

    for (i, &item) in inp1.iter().enumerate() {
        let data : u8 = item ^ inp2[i];
        distance += count_bits(data);
    }
    return distance;
}


fn count_bits(data : u8) -> u32 {
    let mut count = 0;
    let mut inp = data;
    for _i in 0..7 {
        if (inp & 0x01) == 1 {
            count += 1;
        }
        inp = inp >> 1;
    }
    return count;
}


fn fill_two_vectors(rawdata : &Vec<u8>, vector1 : &mut Vec<u8>, vector2 : &mut Vec<u8>) {
    let nb = vector1.len();
    for _i in 0..nb {
        vector1[_i] = rawdata[_i];
    }
    for _i in 0..nb {
        vector2[_i] = rawdata[nb+_i];
    }
}

fn fill_two_vectors2(rawdata : &Vec<u8>, vector1 : &mut Vec<u8>, vector2 : &mut Vec<u8>) {
    let nb = vector1.len();
    for _i in 0..nb {
        vector1[_i] = rawdata[_i + nb];
    }
    for _i in 0..nb {
        vector2[_i] = rawdata[nb + _i + nb];
    }
}


fn fill_blocks(rawdata : &Vec<u8>, liste : &mut Vec<Vec<u8>>) {
    let mut rawdataindex = 0;
    let mut index = 0;
    let mut vector : Vec<u8> = Vec::with_capacity(KEY_SIZE);
    for _i in 0..rawdata.len() {
        if index < KEY_SIZE { // keep filling current vector
            vector.push(rawdata[_i]);
            index += 1;
        } else { // push the current vector and start loading a new one
            liste.push(vector);
            vector = Vec::with_capacity(KEY_SIZE);
            vector.push(rawdata[_i]);
            index = 1;
        }
    }

//    let nbblocks = loop {
//        let mut vector: Vec<u8> = Vec::with_capacity(KEY_SIZE);
//        vector.resize(KEY_SIZE, 0);
//        let ret = fp.read(&mut vector)?;
//        if ret != KEY_SIZE {
//            break counter;
//        }
//        counter += 1;
//        liste.push(vector);
//    };
//    println!("Counted {} blocks", nbblocks);
//    Ok(())
}


fn transpose_blocks(bin : &Vec<Vec<u8>>, bout : &mut Vec<Vec<u8>>) {
    let nbb = bin.len();
    for _i in 0..KEY_SIZE {
        bout[_i] = Vec::new();
        for _j in 0..nbb {
            //println!("[{}, {}] <- {}", _i, _j, bin[_j][_i]);
            bout[_i].push(bin[_j][_i]);
        }
    }
}


// Steps 7 & 8 routines
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
//        print!("[{},{}] ", item.code, item.occurences);
        if item.code != 0 { // Ignoring code 0
            if item.occurences > top.occurences {
                top = item;
            }
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