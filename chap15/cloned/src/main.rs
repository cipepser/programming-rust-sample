fn main() {
    let a = ['1', '2', '3'];

    assert_eq!(a.iter().next(), Some(&'1'));
    assert_eq!(a.iter().cloned().next(), Some('1'));
}