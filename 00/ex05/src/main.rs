fn is_leap_year(year: u32) -> bool {
	if year > 0 && year <= 2024 {
		year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
	}
	else {
		panic!("Invalid");
	}
}

fn num_days_in_month(year: u32, month: u32) -> u32 {
	match month {
		1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
		4 | 6 | 9 | 11 => 30,
		2 => if is_leap_year(year) {
				29
			} else {
				28
			},
		0 | 13..=u32::MAX => panic!("Month has to be between 1-12"),
	}
}

fn num_months_in_year(month: &u32) -> &str {
	match month {
		1 => "January",
		2 => "February",
		3 => "March",
		4 => "April",
		5 => "May",
		6 => "June",
		7 => "July",
		8 => "August",
		9 => "September",
		10 => "October",
		11 => "November",
		12 => "December",
		0 | 13..=u32::MAX => panic!("Month has to be between 1-12"),
	}
}

fn main() {
	let mut c = 5;
	for a in 1..2025 {
		for b in 1..13 {
			while c <= num_days_in_month(a, b) {
				if c == 13 && !(a == 2024 && b > 9) {
					std::println!("Friday, {} {}, {}", num_months_in_year(&b), c, a);
				}
				c += 7;
			}
			c -= num_days_in_month(a, b);
		}
	}
}


#[cfg(test)]
mod tests {
    use crate::*;

	#[should_panic(expected = "Invalid")]
    #[test]
    fn test_is_leap_year() {
        assert_eq!(is_leap_year(1600), true);
        assert_eq!(is_leap_year(1500), false);
        assert_eq!(is_leap_year(2004), true);
        assert_eq!(is_leap_year(2003), false);
        is_leap_year(0);
    }

	#[should_panic(expected = "Month has to be between 1-12")]
    #[test]
    fn test_num_days_in_month() {
        assert_eq!(num_days_in_month(2004, 2), 29);
        assert_eq!(num_days_in_month(2003, 2), 28);
        assert_eq!(num_days_in_month(2004, 1), 31);
        assert_eq!(num_days_in_month(2004, 1), 31);
        assert_eq!(num_days_in_month(2004, 3), 31);
        assert_eq!(num_days_in_month(2004, 3), 31);
        assert_eq!(num_days_in_month(2004, 4), 30);
        assert_eq!(num_days_in_month(2004, 4), 30);
        assert_eq!(num_days_in_month(2004, 5), 31);
        assert_eq!(num_days_in_month(2004, 5), 31);
        assert_eq!(num_days_in_month(2004, 6), 30);
        assert_eq!(num_days_in_month(2004, 6), 30);
        assert_eq!(num_days_in_month(2004, 7), 31);
        assert_eq!(num_days_in_month(2004, 7), 31);
        assert_eq!(num_days_in_month(2004, 8), 31);
        assert_eq!(num_days_in_month(2004, 8), 31);
        assert_eq!(num_days_in_month(2004, 9), 30);
        assert_eq!(num_days_in_month(2004, 9), 30);
        assert_eq!(num_days_in_month(2004, 10), 31);
        assert_eq!(num_days_in_month(2004, 10), 31);
        assert_eq!(num_days_in_month(2004, 11), 30);
        assert_eq!(num_days_in_month(2004, 11), 30);
        assert_eq!(num_days_in_month(2004, 12), 31);
        assert_eq!(num_days_in_month(2004, 12), 31);
		num_days_in_month(7, 0);
		num_days_in_month(2000, 37);
    }
}

