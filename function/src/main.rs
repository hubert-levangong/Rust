struct User {
	username: String,
	email: String,
	age: u32,
	active: bool,
}

fn main() {
    println!("Main function");
    let x = 5;
    let y = {
    	let z = another_function(x);
    	z + 1
    };
    println!("The value for y: {}", y);

    let number = 10;
    if number < 11 {
    	println!("True");
    } else {
    	println!("False");
    }
    let arr = [12,32,54,65,7,12,43];
    array_mul(&arr, 3);

    let mut str1 = String::from("Hello");
    println!("{}, world!", str1);
    //2 examples of code that move str1 to str2
    //let str2 = str1;
    //string_test(str1);


    let str2 = str1.clone();
    string_test(&mut str1);
    println!("{}, super world!", str2);
    println!("{}, mega world!", str1);
    let ret = extract_word(&str1[..]);
    println!("extract_word() returned: {}", ret);

    let user1 = create_user("John".to_string(), "john@doe.net".to_string(), 21);
    println!("user: {}, {}, {}", user1.username, user1.email, user1.age);
}

fn another_function(x: u32) -> u32 {
	let condition = true;
	if condition {
		2 * x
	} else {
		3 * x
	}
}

fn array_mul(t :&[u32], mul :u32) {
	for val in t.iter() {
		println!("{} -> {}", val, val * mul);
	}
}

fn string_test(inp: &mut String) {
	println!("[string_test] input: {}", inp);
	inp.push_str(" more");
}

fn extract_word(inp: &str) -> &str {
	let bytes = inp.as_bytes();
	for (i, &item) in bytes.iter().enumerate() {
		if item == b' ' {
			return &inp[0..i];
		}
	}
	&inp[..]
}

fn create_user(email: String, username: String, age: u32) -> User {
	User {
		username,
		email,
		age,
		active: true,
	}
}