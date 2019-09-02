
fn print_number(x: i32){
	println!("x is {}", x);
}

fn print_sum(x: i32, y: i32){
	println!("sum is {}", x+y);	
}

fn add_one(x: i32) -> i32 {
	x + 1 // no semicolon here!
}

fn main() {
    print_number(5);
    print_sum(5, 2);    

    let mut x: i32 = 1;
    x = add_one(x);
    println!("x is {}", x);

	let f = add_one; //OR let f: fn(i32) -> i32 = add_one; 

	let six = f(5);
	println!("six is {}", six);

}
