struct Point {
    x: f32,
    y: f32,
}

impl Point {
    fn new(x: f32, y: f32) -> Self {
        Point { x, y }
    }
    fn zero() -> Self {
        Point { x: 0.0, y: 0.0 }
    }
    fn distance(&self, other: &Self) -> f32 {
        let a = self.x - other.x;
        let b = self.y - other.y;
        (a * a + b * b).sqrt()
    }
    fn translate(&mut self, dx: f32, dy: f32) {
        self.x += dx;
        self.y += dy;
    }
}

/*
fn main() {
    let mut new = Point::new(5.2, 2.7);
    println!("new: {} {}", new.x, new.y);
    let zero = Point::zero();
    println!("zero: {} {}", zero.x, zero.y);
    let distance = Point::distance(&new, &zero);
    println!("distance: {}", distance);
    Point::translate(&mut new, 2.0, 1.7);
    println!("translate: {} {}", new.x, new.y);
}
*/
