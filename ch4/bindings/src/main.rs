

fn main() {

	// BINDINGS
    let mut x = 5; // mut x: i32

    let (y, z) = (1, 2); // y = 1. z = 2

    let q: i32 = 5;

    // prints
    println!("x = {}", x);
    println!("y = {}", y);
    println!("z = {}", z);
    println!("q = {}", q);

    x = 10; // x is mutable, so we can update it

    println!("x = {}", x);

    // BLOCKS AND SHADOWING
    let i: i32 = 8;
    {
    	println!("{}", i); // Prints 8
    	let i = 12;
    	println!("{}", i); // Prints 12
    }
    println!("{}", i); // Prints 8
    let i = 42;
    println!("{}", i); // Prints 42


    let mut x: i32 = 1;
	x = 7;
	let x = x; // x is now immutable and is bound to 7

	let y = 4;
	let y = "I can also be bound to text!"; // y is now of a different type

    println!("{}", x);
    println!("{}", y);

}
