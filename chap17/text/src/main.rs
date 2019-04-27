fn main() {
    let haystack = "One fine day, in the middle of the night";

    assert_eq!(haystack.find(','), Some(12));
    assert_eq!(haystack.find("night"), Some(35));
    assert_eq!(haystack.find(char::is_whitespace), Some(3));

    assert_eq!("## Elephants"
                   .trim_left_matches(|ch: char| ch == '#' || ch.is_whitespace()),
               "Elephants"
    );

    let code = "\t   function noodle() {";
    assert_eq!(code.trim_left_matches(&[' ', '\t'] as &[char]),
               "function noodle() {"
    );
}
