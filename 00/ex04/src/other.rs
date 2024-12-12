fn main() {
	std::println!("Hey! I'm the other bin target!");
	if !cfg!(debug_assertions) {
		std::println!("I'm in release mode!");
	}
}