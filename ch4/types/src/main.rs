fn main() {

	// Booleans

	let x = true;
	let y: bool = false;

	println!("x = {}",x);
	println!("y = {}",y);

	// Char

	let x = 'x';
	println!("x = {}",x);

	// Numeric

	let x = 42; // x has type i32

	let y = 1.0; // y has type f64

	println!("x = {}",x);
	println!("y = {}",y);


	// Arrays

	let _a = [1, 2, 3]; // a: [i32; 3]

	let mut _m = [1, 2, 3]; // m: [i32, 3] 

	let a = [0; 20]; // a: [i32, 20]

	println!("a has {} elements", a.len());	

	let names = ["Graydon", "Brian", "Niko"]; // names: [&str; 3]
	
	println!("The second name is: {}", names[1]);

	// Slices

	let slice = [0, 1, 2, 3, 4];
	let _complete = &slice[..]; // A slice containing all of the elements in slice
	let _middle = &slice[1..4]; // A slice of slice: only the elements 1, 2, and 3

	// Tuples

	let _x: (i32, &str) = (1, "hello");

	let mut _x = (1, 2); // x: (i32, i32)
	
	let y = (2, 3); // y: (i32, i32)
	
	_x = y;

	let (x, _y, _z) = (1, 2, 3);
	
	println!("x is {}", x);

	let tuple = (11, 2, 3);

	let x = tuple.0;
	let _y = tuple.1;
	let _z = tuple.2;

	println!("x is {}", x);

}
