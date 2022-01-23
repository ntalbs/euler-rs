fn is_leap_year(year: u16) -> bool {
    if year % 100 == 0 {
        if year % 400 == 0 {
            return true
        } else {
            return year % 4 == 0
        }
    }
    false
}

fn days_in_month(year: u16, month: u8) -> u8 {
    assert!(1 <= month && month <= 12);
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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
}

struct DateIter {
    current: Date,
}

impl DateIter {
    fn new(start: Date) -> DateIter {
        DateIter {
            current: start,
        }
    }
}

impl Iterator for DateIter {
    type Item = Date;
    fn next(&mut self) -> Option<Date> {
        let current = &self.current;
        let next_dw = (current.day_of_week + 1) % 7;
        
        let next_date = if current.day_of_month < days_in_month(current.year, current.month) {
            Date::new(current.year, current.month, current.day_of_month + 1, next_dw)
        } else if current.month < 12 {
            Date::new(current.year, current.month + 1, 1, next_dw)
        } else {
            Date::new(current.year + 1, 1, 1, next_dw)
        };
        self.current = next_date;
        Some(next_date)
    }
}

pub fn sol() -> u64 {
    DateIter::new(Date::new(1900, 1, 1, 1))
        .skip_while(|d| d.year <= 1900)
        .take_while(|d| d.year <= 2000)
        .filter(|d| d.day_of_month == 1 && d.day_of_week == 0)
        .count() as u64
}

#[test]
fn test() {
    assert_eq!(sol(), 171);
}
