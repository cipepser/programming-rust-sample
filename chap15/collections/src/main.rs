use std::collections::{HashSet, BTreeSet, LinkedList, HashMap, BTreeMap};

fn main() {
    let args: HashSet<String> = std::env::args().collect();
    let args: BTreeSet<String> = std::env::args().collect();
    let args: LinkedList<String> = std::env::args().collect();

    // Collecting a map requires (key, value) pairs, so for this example,
    // zip the sequence of strings with a sequence of integers.
    let args: HashMap<String, usize> = std::env::args().zip(0..).collect();
    let args: BTreeMap<String, usize> = std::env::args().zip(0..).collect();
}
