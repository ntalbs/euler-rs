fn is_leap_year(year: u16) -> bool {
    if year % 100 == 0 {
        return if year % 400 == 0 { true } else { year % 4 == 0 };
    }
    false
}

fn days_in_month(year: u16, month: u8) -> u8 {
    assert!((1..=12).contains(&month));
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        _ => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
    }
}

#[derive(Debug)]
struct Date {
    year: u16,
    month: u8,
    day_of_month: u8,
    day_of_week: u8,
}

impl Date {
    fn new(year: u16, month: u8, day_of_month: u8, day_of_week: u8) -> Self {
        Date {
            year,
            month,
            day_of_month,
            day_of_week,
        }
    }

    fn next(&mut self) {
        let next_dw = (self.day_of_week + 1) % 7;

        if self.day_of_month < days_in_month(self.year, self.month) {
            self.day_of_month += 1;
        } else if self.month < 12 {
            self.month += 1;
            self.day_of_month = 1;
        } else {
            self.year += 1;
            self.month = 1;
            self.day_of_month = 1;
        }
        self.day_of_week = next_dw;
    }
}

pub fn sol() -> i64 {
    let mut count: i64 = 0;
    let mut d = Date::new(1900, 1, 1, 1);
    loop {
        d.next();
        if d.year <= 1900 {
            continue;
        }
        if d.year > 2000 {
            break;
        }
        if d.day_of_month == 1 && d.day_of_week == 0 {
            count += 1;
        }
    }
    count
}

#[test]
fn test() {
    assert_eq!(sol(), 171);
}
