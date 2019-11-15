fn main() {
    let in1 = Vec::from("this is a test");
    let in2 = Vec::from("wokka wokka!!!");
    let res = hamming_distance(&in1, &in2);

    println!("Hamming distance = {}", res);
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
    for _i in (0..7).rev() {
        if (inp & 0x01) == 1 {
            count += 1;
        }
        inp = inp >> 1;
    }
    return count;
}