use std::ops::{Deref, DerefMut};

struct Selector<T> {
    /// Elements available in this `Selector`.
    elements: Vec<T>,

    /// The index of the "current" element in `elements`.
    /// A `Selector behaves like a pointer to the current element.
    current: usize,
}

impl<T> Deref for Selector<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.elements[self.current]
    }
}

impl<T> DerefMut for Selector<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.elements[self.current]
    }
}

fn main() {
    let mut s = Selector { elements: vec!['x', 'y', 'z'], current: 2 };

    // Because `Selector implements `Deref`, we can use the `*` operator
    // to refer to its current element.
    assert_eq!(*s, 'z');

    // Assert that 'z' is alphabetic, useing a method of `char`
    // directly on a `Selector`, via deref coercion.
    assert!(s.is_alphabetic());

    // Change the 'z' to 'w', by assigning to the `Selector`'s  referent.
    *s = 'w';
    assert_eq!(s.elements, ['x', 'y', 'w']);
}
