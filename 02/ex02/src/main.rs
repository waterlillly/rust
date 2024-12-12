enum PizzaStatus {
    Ordered,
    Cooking,
    Cooked,
    Delivering,
    Delivered,
}

impl PizzaStatus {
    fn from_delivery_time(ordered_days_ago: u32) -> Self {
		PizzaStatus {
			Ordered: ordered_days_ago,
			Cooking: 2,
			Cooked: 5,
			Delivering: 3,
			Delivered: 7,
		}
	}
    fn get_delivery_time_in_days(&self) -> u32 {
		let x: u32 = self.Ordered;
		let y: u32 = self.Cooking + self.Cooked + self.Delivering + self.Delivered;
		if x > y {
			x - y;
		} else {
			y - x;
		}
	}
}

fn main() {
	let pizza = PizzaStatus::from_delivery_time(17);
	let delivery = PizzaStatus::get_delivery_time_in_days(&pizza);
    println!("delivery: {}", delivery);
}
