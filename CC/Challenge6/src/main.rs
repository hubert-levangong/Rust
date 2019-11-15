use std::fs;
use std::io::Read;
use std::error::Error;

#[derive(Copy, Clone)]
struct Distance {
    keysz : usize,
    dist: f32,
}

// Top 3 distances with corresponding key sizes
// [14 : 3.9285715]
// [26 : 3.8461537]
// [12 : 3.8333333]

fn main() {
//    let in1 = Vec::from("this is a test");
//    let in2 = Vec::from("wokka wokka!!!");
//    let res = hamming_distance(&in1, &in2);
//    println!("Hamming distance = {}", res);
    let mut dists= Vec::new();
    keysize_distances(2, 40, &mut dists);
    println!("result has {} distances", dists.len());
    for &elem in dists.iter() {
        println!("[{}:{}]", elem.keysz, elem.dist);
    }

}


fn keysize_distances(keysizemin : usize, keysizemax : usize, distances : &mut Vec<Distance>) {
    println!("key sizes in ({}, {})", keysizemin, keysizemax);
    for keysize in keysizemin..keysizemax+1 {
        let mut in1 : Vec<u8> = Vec::with_capacity(keysize);
        in1.resize(keysize, 0);
        let mut in2 : Vec<u8> = Vec::with_capacity(keysize);
        in2.resize(keysize, 0);
        fill_two_vectors(&mut in1, &mut in2).expect("Could not fill vectors with data.");
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


fn fill_two_vectors(vector1 : &mut Vec<u8>, vector2 : &mut Vec<u8>) -> Result<(), Box<dyn Error>> {
    let filename = String::from("/home/hubert/Documents/Github/Rust/CC/Challenge6/src/6.txt");
    let mut fp = fs::File::open(filename)?;

    fp.read(vector1)?;
    fp.read(vector2)?;

    Ok(())
}