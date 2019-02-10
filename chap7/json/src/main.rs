fn doSomething() -> Result<(), JsonError> {
    return Err(JsonError {
        message: "expectd ']' at end of array".to_string(),
        line: current_line,
        column: current_column,
    });
}

fn main() {
    println!("Hello, world!");
}
