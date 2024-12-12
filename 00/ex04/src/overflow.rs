fn main() {
	let mut a: u8 = 255;
	let b: u8 = a;
	a += 1 ;
	std::println!("{}u8 + 1u8 == {}", b, a);
}