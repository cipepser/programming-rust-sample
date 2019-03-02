fn main() {
    let mut last = 0;
    for i in 0..100 {
        last = i;
    }
    assert_eq!(last, 99);

    match 100 {
        0...100 => println!("match"),
        _ => println!("unmatched"),
    }
}
