//use std::fmt::Display;

//trait Being<T: Display> {
//    fn speak(T);
//}
//
//struct Person;
//
//impl Being for Person {
//    fn speak(s: &str) {
//        println!("{}", s);
//    }
//}

//struct Japanese;
//
//impl Person for Japanese {
//    fn speak() {};
//}
//
//struct American;
//
//
////trait Handler<W: Writer> {
////    fn handle(&self, &mut W) -> IoResult<()>;
////}
////
////
////fn take_str<T:?Sized>(_s: &T) {
////}

#[derive(Debug)]
struct S<T: ?Sized> {
    b: Box<T>
}

fn main() {
    let s = S { b: Box::new("a".to_string()) };
    println!("{:?}", s);
    println!("{:?}", as_raw_bytes(&s));

    let c = "alice".to_string();
    let size_of_val = std::mem::size_of_val(&c);
    println!("{:?}", as_raw_bytes(&c));
    println!("size_of_val: {}", size_of_val);
    println!("size_of: {}", std::mem::size_of::<bool>());
}

//fn main() {
////    let s = "hello!";
////    take_str(s);
//}


fn as_raw_bytes<'a, T: ?Sized>(x: &'a T) -> &'a [u8] {
    unsafe {
        std::slice::from_raw_parts(
            x as *const T as *const u8,
            std::mem::size_of_val(x))
    }
}
