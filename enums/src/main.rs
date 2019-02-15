use std::mem::size_of;

enum Pet {
    Orca,
    Giraffe,
}

#[derive(Debug, PartialEq)]
enum HttpStatus {
    Ok = 200,
    NotModified = 304,
    NotFound = 404,
}

fn http_status_from_u32(n: u32) -> Option<HttpStatus> {
    match n {
        200 => Some(HttpStatus::Ok),
        304 => Some(HttpStatus::NotModified),
        404 => Some(HttpStatus::NotFound),
        _ => None
    }
}

fn main() {
    assert_eq!(size_of::<Pet>(), 1);
    assert_eq!(size_of::<HttpStatus>(), 2); // 404 doesn't fit in a u8

    assert_eq!(HttpStatus::Ok as i32, 200);

    assert_eq!(http_status_from_u32(200).unwrap(), HttpStatus::Ok);
}