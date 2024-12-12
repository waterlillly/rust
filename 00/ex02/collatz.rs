fn collatz(start: u32) {
    let mut x: u32 = start;
    if x > 0 {
        std::println!("{}", x);
        while x != 1 {
            if x % 2 == 0 {
                x /= 2;
                std::println!("{}", x);
            } else {
                x *= 3;
                x += 1;
                std::println!("{}", x);
            }
        }
    }
}

/*
fn main() {
    collatz(1);
}
*/
