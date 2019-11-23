#![feature(trace_macros)]

fn main() {
    trace_macros!(true);
    let numbers = vec![1, 2, 3];
    trace_macros!(false);
    println!("total: {}", numbers.iter().sum::<u64>());
}

//❯ cargo +nightly run
//   Compiling debug v0.1.0 (/Users/cipepser/.go/src/github.com/cipepser/programming-rust-sample/chap20/debug)
//note: trace_macro
// --> src/main.rs:5:19
//  |
//5 |     let numbers = vec![1, 2, 3];
//  |                   ^^^^^^^^^^^^^
//  |
//  = note: expanding `vec! { 1, 2, 3 }`
//  = note: to `< [_] > :: into_vec (box [1, 2, 3])`
//
//    Finished dev [unoptimized + debuginfo] target(s) in 0.96s
//     Running `target/debug/debug`
//total: 6