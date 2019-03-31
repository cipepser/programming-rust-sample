use std::iter::Peekable;

fn parse_number<I>(tokens: &mut Peekable<I>) -> u32
    where I: Iterator<Item=char>
{
    let mut n = 0;
    loop {
        match tokens.peek() {
            Some(r) if r.is_digit(10) => {
                n = n * 10 + r.to_digit(10).unwrap();
            }
            _ => return n,
        }
        tokens.next();
    }
}

fn main() {
    let mut chars = "123456789,987654321".chars().peekable();
    assert_eq!(parse_number(&mut chars), 123456789);

    // `parse_number` didn't comsume the comma.
    assert_eq!(chars.next(), Some(','));
    assert_eq!(parse_number(&mut chars), 987654321);
    assert_eq!(chars.next(), None);
}
