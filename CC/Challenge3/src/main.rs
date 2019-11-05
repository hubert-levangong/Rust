extern crate hex;

#[derive(Copy, Clone)]
struct Freq {
    code : u8,
    occurences: i32,
}

fn main() {
    let test = hex::decode("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736").expect("Failed to decode hex string");
    println!("data has {} characters.", test.len());
    get_most_frequent_code(&test);

}


fn get_most_frequent_code(data : &Vec<u8>) -> i32 {
    let mut liste : Vec<Freq> = Vec::new();
    let mut res = 0;

    // Calculate frequencies
    for (i, &item) in data.iter().enumerate() {
        let c = item;
        res = 0;
        for (j, &item2) in data.iter().enumerate() {
            if !in_liste(item2, &liste) {
                println!("comparing [{}]-[{}]", c, item2);
                if c == item2 {
                    res += 1;
                }
                let elem : Freq = Freq {code: c, occurences: res};
                liste.push(elem);
            } else { println!("skipping [{}]", item2);}
        }
    }
    println!("The data contains {} different codes", liste.len());
    print!("Liste: ");
    for (j, &item3) in liste.iter().enumerate() {
        print!("[{}, {}]", item3.code, item3.occurences);
    }

    return res;
}


fn in_liste(code : u8, liste : &Vec<Freq>) -> bool {
    for (i, elem) in liste.iter().enumerate() {
        if code == elem.code { return true}
    }
    return false;
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