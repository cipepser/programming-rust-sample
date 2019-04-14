fn main() {
    let a = [5, 6, 7, 8, 9, 10];

    assert_eq!(a.iter().fold(0, |n, _| n + 1), 6);
    assert_eq!(a.iter().fold(0, |n, i| n + i), 45);
    assert_eq!(a.iter().fold(1, |n, i| n * i), 151200);

    // max
    assert_eq!(a.iter().fold(i32::min_value(), |m, &i| std::cmp::max(m, i)), 10);

    // We can use uncopiable type as an accumulator.
    let a = ["Pack ", "my ", "box ", "with ", "five ", "dozen ", "liquor ", "jugs"];

    let pangram = a.iter().fold(String::new(),
                                |mut s, &w|
                                    {
                                        s.push_str(w);
                                        s
                                    });
    assert_eq!(pangram, "Pack my box with five dozen liquor jugs");
}
