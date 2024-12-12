fn min<'a>(a: &'a i32, b: &'a i32) -> &'a i32 {
	if *a < *b {
		a
	} else {
		b
	}
}

/*
fn main() {
	let a: &i32 = &5;
	let b: &i32 = &2;
    let x: &i32 = min(a, b);
	println!("x: {}", *x);
}
*/