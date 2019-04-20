fn main() {
    let mut v = vec![0, 10, 20, 30];

    v.insert(2, 15);
    assert_eq!(v, vec![0, 10, 15, 20, 30]);

    v.remove(2);
    assert_eq!(v, vec![0, 10, 20, 30]);
}
