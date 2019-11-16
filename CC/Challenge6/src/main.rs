use std::fs;
use std::io::Read;
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
// 3 smallest distances with corresponding key sizes
// [2 : 2.5]
// [3 : 2]
// [5 : 1.2] <--- key size of 5 bytes

const KEY_SIZE: usize = 5;

fn main() {
//    // Steps 1 & 2
//    let in1 = Vec::from("this is a test");
//    let in2 = Vec::from("wokka wokka!!!");
//    let res = hamming_distance(&in1, &in2);
//    println!("Hamming distance = {}", res);

    // First we read the data from the file, remove EOL chars and Base64 decode it
    let filename = String::from("/home/hubert/Documents/Github/Rust/CC/Challenge6/src/6.txt");
    let mut fp = fs::File::open(filename).expect("Could not open the file");
    let mut b64data = String::new();
    let nbread = fp.read_to_string(&mut b64data).expect("Could not read the file");
    let mut b64noEOLdata = b64data.replace("\n", "");
    let rawdata = decode_config(&b64noEOLdata, base64::STANDARD).expect("Could not Base64 decode the data");
    println!("Base64 decoded data is {} bytes long", rawdata.len());

//    // Steps 3 & 4
//    let mut dists= Vec::new();
//    keysize_distances(&rawdata,2, 40, &mut dists);
//    println!("result has {} distances", dists.len());
//    for &elem in dists.iter() {
//        println!("[{}:{}]", elem.keysz, elem.dist);
//    }

    // Steps 5 & 6
    let mut blocks : Vec<Vec<u8>> = Vec::new();
    fill_blocks(&rawdata,&mut blocks);
    println!("Blocks: {:?}", blocks);
    let bl = blocks.len();
    let mut tblocks : Vec<Vec<u8>> = Vec::with_capacity(KEY_SIZE);
    tblocks.resize(KEY_SIZE, Vec::new());
    transpose_blocks(&blocks, &mut tblocks);
    println!("Transposed Blocks: {:?}", tblocks);

// Steps 7 & 8

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