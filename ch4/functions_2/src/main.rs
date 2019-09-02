
fn main() {

	let x = 6;

	// if 
	if x == 5 {
		println!("x is five!");
	} else if x == 6 {
		println!("x is six!");
	} else {
		println!("x is not five or six :(");
	}
	
	let x = 5;
	
	let y = if x == 5 { 10 } else { 15 }; // y: i32
	
	println!("y is {}", y);

	// loops

	let mut x = 5; // mut x: i32
	let mut done = false; // mut done: bool
	
	while !done {
		x += x - 3;

		println!("{}", x);

		if x % 5 == 0{
			done = true;
		}
	}

	for x in 0..5 {
		println!("loop: {}", x); // x: i32
	}

	// enumerate

	for (index, value) in (5..10).enumerate(){
		println!("index = {} and value = {}", index, value);
	}

	let lines = "hello\nworld".lines();

	for (linenumber, line) in lines.enumerate() {
		println!("{}: {}", linenumber, line);
	}

	// ending iterations early 

	let mut x = 5;

	loop {
		x += x - 3;

		println!("loop: {}", x);

		if x % 5 == 0 { break; }
	}

	for x in 0..10 {
		if x % 2 == 0 { continue; }
		
		println!("{}", x);
	}

}
