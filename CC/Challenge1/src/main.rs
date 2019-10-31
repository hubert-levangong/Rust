

fn main() {
    println!("Starting");
    let mut input = Vec::new();
    let mut val : u8;
    val = 'M' as u8;
    input.push(val);
    val = 'a' as u8;
    input.push(val);
    val = 'n' as u8;
    input.push(val);
    println!("calling function with {:?}", input);
    println!("response was: {}", hex_to_base64(&mut input));
}

fn hex_to_base64(input : &mut Vec<u8>) -> String {
    let res = String::new();

    return res;
}