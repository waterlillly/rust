fn print_bytes(s: &str) {
    for x in s.bytes() {
        std::println!("{}", x);
    }
}

/*
fn main() {
    print_bytes("");
}
*/
