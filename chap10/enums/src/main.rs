use std::mem::size_of;

enum Pet {
    Orca,
    Giraffe,
}

#[derive(Debug, PartialEq)]
enum HttpStatus {
    Ok = 200,
    NotModified = 304,
    NotFound = 404,
}

fn http_status_from_u32(n: u32) -> Option<HttpStatus> {
    match n {
        200 => Some(HttpStatus::Ok),
        304 => Some(HttpStatus::NotModified),
        404 => Some(HttpStatus::NotFound),
        _ => None
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum TimeUnit {
    Seconds,
    Minutes,
    Hours,
    Days,
    Months,
    Years,
}

impl TimeUnit {
    /// Return the plural noun for this time unit.
    fn plural(self) -> &'static str {
        match self {
            TimeUnit::Seconds => "seconds",
            TimeUnit::Minutes => "minutes",
            TimeUnit::Hours => "hours",
            TimeUnit::Days => "days",
            TimeUnit::Months => "months",
            TimeUnit::Years => "years",
        }
    }

    /// Return the singular noun for this time unit.
    fn singular(self) -> &'static str {
        self.plural().trim_right_matches('s')
    }
}

fn main() {
    assert_eq!(size_of::<Pet>(), 1);
    assert_eq!(size_of::<HttpStatus>(), 2); // 404 doesn't fit in a u8

    assert_eq!(HttpStatus::Ok as i32, 200);

    assert_eq!(http_status_from_u32(200).unwrap(), HttpStatus::Ok);

    assert_eq!(TimeUnit::Seconds.plural(), "seconds");
    assert_eq!(TimeUnit::Seconds.singular(), "second");
}