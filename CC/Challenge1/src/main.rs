extern crate hex;


fn main() {
    //let mut input = Vec::from("I'm killing your brain like a poisonous mushroom");
    let mut input = hex::decode("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d").expect("Decoding failed");
    let mut res = String::new();
    hex_to_base64(&mut input, &mut res);
    println!("Base 64 value is: {}", res);
}

fn hex_to_base64(input : &mut Vec<u8>, output : &mut String) {
    while input.len() > 0 {
        if input.len() >= 3 {
            // 1st
            let mut c: u8 = input[0];
            c = c >> 2;
            B64Value(&mut c);
            output.push(c as char);

            // 2nd
            let mut m = input[0];
            m = m & 0x03;
            m = m << 4;
            c = input[1];
            c = c >> 4;
            c = m | c;
            B64Value(&mut c);
            output.push(c as char);

            // 3rd
            m = input[1];
            m = m & 0x0F;
            m = m << 2;
            c = input[2];
            c = c >> 6;
            c = m | c;
            B64Value(&mut c);
            output.push(c as char);

            // 4th
            c = input[2];
            c = c & 0x3F;
            B64Value(&mut c);
            output.push(c as char);
            input.remove(0);
            input.remove(0);
            input.remove(0);
        } else if input.len() == 2 {
            let mut c: u8 = input[0];
            c = c >> 2;
            B64Value(&mut c);
            output.push(c as char);

            let mut m = input[0];
            m = m & 0x03;
            m = m << 4;
            c = input[1];
            c = c >> 4;
            c = m | c;
            B64Value(&mut c);
            output.push(c as char);

            c = input[1];
            c = c & 0x0F;
            c = c << 2;
            B64Value(&mut c);
            output.push(c as char);

            // Padding =
            c = 61;
            output.push(c as char);
            input.remove(0);
            input.remove(0);
        } else {
            let mut c: u8 = input[0];
            c = c >> 2;
            B64Value(&mut c);
            output.push(c as char);

            c = input[0];
            c = c & 0x3;
            c = c << 4;
            B64Value(&mut c);
            output.push(c as char);

            // Padding ==
            c = 61;
            output.push(c as char);
            output.push(c as char);
            input.remove(0);
        }
    }
}


fn B64Value(inp : &mut u8) {
    let c : u8 = *inp;
    if c == 62 { // '+'
        *inp = 43;
    } else if c == 63 { // '/'
        *inp = 47;
    } else if c < 26 { // 'A-Z'
        *inp += 65;
    } else if c < 52 { // 'a-z'
        *inp += 71;
    } else { // '0-9'
        *inp -= 4;
    }
}