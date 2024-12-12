fn min(a: i32, b: i32) -> i32 {
    if a < b {
        a
    } else {
        b
    }
}

fn main() {
    let x: i32 = min(11, 10);
    std::println!("min (10, 11): {}", x);
    let y: i32 = min(5, 5);
    std::println!("min (5, 5): {}", y);
    let z: i32 = min(5, 0);
    std::println!("min (5, 0): {}", z);
}
