fn main() {
    for x in 1..101 {
        match (x % 11, x % 3, x % 5) {
            (_, 0, 0) => std::println!("fizzbuzz"),
            (_, 0, _) => std::println!("fizz"),
            (_, _, 0) => std::println!("buzz"),
            (3, _, _) => std::println!("FIZZ"),
            (5, _, _) => std::println!("BUZZ"),
            (_, _, _) => std::println!("{}", x),
        }
    }
}
