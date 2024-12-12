fn add(a: &i32, b: i32) -> i32 {
	a + b
}

fn add_assign(a: &mut i32, b: i32) {
	*a = add(a, b);
}

/*
fn main() {
    let mut a: i32 = 3;
	let b: i32 = 5;
	add_assign(&mut a, b);
	println!("a: {} b: {}", a, b);
	add_assign(&mut a, b);
	println!("a: {} b: {}", a, b);
	add_assign(&mut a, b);
	println!("a: {} b: {}", a, b);
}
*/