#[derive(Debug)]
struct Rectangle {
	width: u32,
	height: u32,
}

impl Rectangle {
	// method 1
	fn area(&self) -> u32 {
		self.width * self.height
	}

	// method 2
	fn can_hold(&self, rect: &Rectangle) -> bool {
		self.width > rect.width && self.height > rect.height
	}

	// associated function 1
	fn square(size: u32) -> Rectangle {
		Rectangle { width: size, height: size }
	}
}

fn main() {
	let rect1 = Rectangle {width: 30, height: 50 };
	let rect2 = Rectangle {width: 10, height: 40 };
	let rect3 = Rectangle {width: 60, height: 45 };
	println!("rect1 is {:?}", rect1);
    println!("Rectangle Area: {}", rect1.area());

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let sq = Rectangle::square(3);
    println!("square: {:?}", sq);
}
