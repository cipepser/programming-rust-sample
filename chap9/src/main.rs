/// A rectangle of eight-bit grayscale pixels.
struct GrayscaleMap {
    pixels: Vec<u8>,
    size: (usize, usize),
}

fn new_map(size: (usize, usize), pixels: Vec<u8>) -> GrayscaleMap {
    assert_eq!(pixels.len(), size.0 * size.1);
    GrayscaleMap { pixels, size }
}

struct Broom {
    name: String,
    height: u32,
    health: u32,
    position: (f32, f32, f32),
    intent: BroomIntent,
}

/// Two possible alternatives for what a `Broom` could be working on.
#[derive(Copy, Clone)]
enum BroomIntent { FetchWater, DumpWater }

// Receive the input room by value, taking ownership.
fn chop(b: Broom) -> (Broom, Broom) {
    // Initialize `broom1` mostly from `b`, changing only `height`.
    // Since `String` is not `Copy`, `broom1` takes ownership of `b`'s name.
    let mut broom1 = Broom { height: b.height / 2, ..b };

    // Initialize `broom2` mostly from `broom1`.
    // Since `String` is not `Copy`, we must clone `name` explicitly.
    let mut broom2 = Broom {
        name: broom1.name.clone(),
        ..broom1
    };

    broom1.name.push_str(" I");
    broom2.name.push_str(" II");

    (broom1, broom2)
}

/// A first-in, first-out queue of characters.
pub struct Queue<T> {
    older: Vec<T>,
    // older elements, eldest last.
    younger: Vec<T>, // younger element, youngest last.
}

impl<T> Queue<T> {
    /// Push a character onto the back of a queue.
    pub fn push(&mut self, t: T) {
        self.younger.push(t);
    }

    /// Pop a character off the front of a queue. Return `Some(c)` if there
    /// was a character to pop, or `None` if the queue was empty.
    pub fn pop(&mut self) -> Option<T> {
        if self.older.is_empty() {
            if self.younger.is_empty() {
                return None;
            }

            // Bring the elements in youger over to older, and put them in the promised order.
            use std::mem::swap;
            swap(&mut self.older, &mut self.younger);
            self.older.reverse();
        }

        // Now older is guaranteed to have something. Vec's pop method
        // already returns an Option, so we're set.
        self.older.pop()
    }

    pub fn is_empty(&self) -> bool {
        self.older.is_empty() && self.younger.is_empty()
    }

    pub fn split(self) -> (Vec<T>, Vec<T>) {
        (self.older, self.younger)
    }

    pub fn new() -> Queue<T> {
        Queue {
            older: Vec::new(),
            younger: Vec::new(),
        }
    }
}

fn main() {
//    // GrayscaleMap
//    let width = 1024;
//    let height = 576;
//    let image = GrayscaleMap {
//        pixels: vec![0; width * height],
//        size: (width, height),
//    };
//    assert_eq!(image.size, (1024, 576));
//    assert_eq!(image.pixels.len(), 1024 * 576);

//    // Broom
//    let hokey = Broom {
//        name: "Hokey".to_string(),
//        height: 60,
//        health: 100,
//        position: (100.0, 200.0, 0.0),
//        intent: BroomIntent::FetchWater,
//    };
//
//    let (hokey1, hokey2) = chop(hokey);
//    assert_eq!(hokey1.name, "Hokey I");
//    assert_eq!(hokey1.health, 100);
//
//    assert_eq!(hokey2.name, "Hokey II");
//    assert_eq!(hokey2.health, 100);

    // Queue
//    let mut q = Queue {
//        older: Vec::new(),
//        younger: Vec::new(),
//    };
    let mut q = Queue::new();

    q.push('0');
    q.push('1');
    assert_eq!(q.pop(), Some('0'));

    q.push('∞');
    assert_eq!(q.pop(), Some('1'));
    assert_eq!(q.pop(), Some('∞'));
    assert_eq!(q.pop(), None);

    assert!(q.is_empty());
    q.push('x');
    assert!(!q.is_empty());
    q.pop();

    q.push('P');
    q.push('D');
    assert_eq!(q.pop(), Some('P'));
    q.push('X');

    let (older, younger) = q.split();
    // q is now uninitialized.
    assert_eq!(older, vec!['D']);
    assert_eq!(younger, vec!['X']);
}
