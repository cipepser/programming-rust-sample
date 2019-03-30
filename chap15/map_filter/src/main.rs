fn main() {
    // no output, because it does not call `next` method.
    ["a", "b", "c"].iter().map(|elt| println!("{}", elt));
}
