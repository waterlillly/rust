fn choose<T>(values: &[T]) -> &T {
	if values.is_empty() {
		panic!("empty");
	} else if values.len() == 1 {
		&values[0]
	} else {
		values.random_number()
	} 
}

fn main() {
   println!("picked: {}", choose([0, 6, 45, 9, 3]));
}
