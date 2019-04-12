use std::cmp::{PartialOrd, Ordering};

fn cmp(lhs: &&f64, rhs: &&f64) -> Ordering {
    lhs.partial_cmp(rhs).unwrap()
}

fn main() {
    let numbers = [1.0, 2.0, 4.0];
    assert_eq!(numbers.iter().max_by(cmp), Some(&4.0));
    assert_eq!(numbers.iter().min_by(cmp), Some(&1.0));

//    let numbers = [1.0, 2.0, std::f64::NAN, 4.0];
//    assert_eq!(numbers.iter().max_by(cmp), Some(&4.0)); // panic
}
