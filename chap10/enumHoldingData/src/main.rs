/// A timestamp that has been deliberately rounded off, so our program
/// says "6 months ago" insterad of "February 9, 2016, at 9:49 AM"
#[derive(Copy, Clone, Debug, PartialEq)]
enum RoughTime {
    InThePast(TimeUnit, u32),
    JustNow,
    InTHeFuture(TimeUnit, u32),
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
        RoughTime::InThePast(units, 1) => format!("a {} ago", units.plural()),
        RoughTime::InThePast(units, count) => format!("{} {} ago", count, units.singular()),
        RoughTime::JustNow => format!("just now"),
        RoughTime::InTHeFuture(units, 1) => format!("a {} from now", units.plural()),
        RoughTime::InTHeFuture(units, count) => format!("{} {} from now", count, units.singular()),
    }
}

fn main() {
    let four_score_and_seven_years_ago = RoughTime::InThePast(TimeUnit::Years, 4 * 20 + 7);
    println!("{:?}", rough_time_to_english(four_score_and_seven_years_ago));

    let three_hours_from_now = RoughTime::InThePast(TimeUnit::Hours, 3);
    println!("{:?}", rough_time_to_english(three_hours_from_now));
}
