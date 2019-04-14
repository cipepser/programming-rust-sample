use std::cmp::{PartialOrd, Ordering};
use std::collections::HashMap;

fn cmp(lhs: &&f64, rhs: &&f64) -> Ordering {
    lhs.partial_cmp(rhs).unwrap()
}

fn main() {
    let numbers = [1.0, 2.0, 4.0];
    assert_eq!(numbers.iter().max_by(cmp), Some(&4.0));
    assert_eq!(numbers.iter().min_by(cmp), Some(&1.0));

//    let numbers = [1.0, 2.0, std::f64::NAN, 4.0];
//    assert_eq!(numbers.iter().max_by(cmp), Some(&4.0)); // panic

    let mut populations = HashMap::new();
    populations.insert("Portland", 583_776);
    populations.insert("Fossil", 449);
    populations.insert("Greenhorn", 2);
    populations.insert("Boring", 7_762);
    populations.insert("The Dalles", 15_340);

    assert_eq!(populations.iter().max_by_key(|&(_name, pop)| pop), Some((&"Portland", &583_776)));
    assert_eq!(populations.iter().min_by_key(|&(_name, pop)| pop), Some((&"Greenhorn", &2)));

    let packed = "Helen of Troy";
    let spaced = "Helen   of   Troy";
    let obscure = "Helen of Sandusky";

    assert!(packed != spaced);
    assert!(packed.split_whitespace().eq(spaced.split_whitespace()));

    assert!(spaced < obscure);
    assert!(spaced.split_whitespace().gt(obscure.split_whitespace()));
}
