/// A timestamp that has been deliberately rounded off, so our program
/// says "6 months ago" insterad of "February 9, 2016, at 9:49 AM"
#[derive(Copy, Clone, Debug, PartialEq)]
enum RoughTime {
    InThePast(TimeUnit, u32),
    JustNow,
    InTheFuture(TimeUnit, u32),
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

fn rough_time_to_english(rt: RoughTime) -> String {
    match rt {
        RoughTime::InThePast(TimeUnit::Hours, 1) => format!("an hour ago"),
        RoughTime::InThePast(units, 1) => format!("a {} ago", units.singular()),
        RoughTime::InThePast(units, count) => format!("{} {} ago", count, units.plural()),
        RoughTime::JustNow => format!("just now"),
        RoughTime::InTheFuture(TimeUnit::Hours, 1) => format!("an hour from now"),
        RoughTime::InTheFuture(units, 1) => format!("a {} from now", units.singular()),
        RoughTime::InTheFuture(units, count) => format!("{} {} from now", count, units.plural()),
    }
}

fn main() {
    assert_eq!(rough_time_to_english(RoughTime::InThePast(TimeUnit::Years, 4 * 20 + 7)), "87 years ago".to_string());
    assert_eq!(rough_time_to_english(RoughTime::InThePast(TimeUnit::Hours, 3)), "3 hours ago".to_string());
    assert_eq!(rough_time_to_english(RoughTime::InThePast(TimeUnit::Months, 1)), "a month ago".to_string());
    assert_eq!(rough_time_to_english(RoughTime::InThePast(TimeUnit::Hours, 1)), "an hour ago".to_string());

    assert_eq!(rough_time_to_english(RoughTime::InTheFuture(TimeUnit::Months, 1)), "a month from now".to_string());
    assert_eq!(rough_time_to_english(RoughTime::InTheFuture(TimeUnit::Hours, 1)), "an hour from now".to_string());
}
