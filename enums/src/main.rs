use std::mem::size_of;

enum Pet {
    Orca,
    Giraffe,
}

enum HttpStatus {
    Ok = 200,
    NotModified = 304,
    NotFound = 404,
}

fn main() {
    assert_eq!(size_of::<Pet>(), 1);
    assert_eq!(size_of::<HttpStatus>(), 2); // 404 doesn't fit in a u8
}