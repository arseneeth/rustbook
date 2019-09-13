fn main() {
    let mut v = vec![1, 2, 3, 4, 5]; // v: Vec<i32>

    let vv = vec![0;10]; // ten zeroes

    println!("The third element of vv is {}", vv[2]);
    println!("The third element of v is {}", v[2]);

    // It doesn't work
	// let j: i32 = 0; 
	// v[j];

	// It does
	let i: usize = 1;
	println!("The second element of v is {}", v[i]);

	match vv.get(7){
		Some(x) => println!("Item 7 is {}", x),
		None => println!("Sorry, this vector is too short.")
	}

	for i in &vv {
		println!("A reference to {}", i);
	}

	for i in &mut v {
		println!("A mutable reference to {}", i);
	}

	for i in v {
		println!("Take ownership of the vector and its element {}", i);
	}

}
