fn main() {
    use std::str::FromStr;

    let text = "1\nfrond .25 289\n3.1415 estuary\n";
    for number in text.split_whitespace()
        .filter_map(|w| f64::from_str(w).ok()) {
        println!("{:4.2}", number.sqrt())
    }
}
