use std::fs;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let hometown = "Tokyo";
    match get_weather(hometown) {
        Ok(report) => {
            println!("{}: {}", hometown, report);
        }
        Err(err) => {
            println!("error querying the weather: {}", err);
        }
    }
}

fn get_weather(hometown: &str) -> Result<&str, String> {
    match hometown {
        "Tokyo" => Ok("Sunny!"),
        _ => Err("no location".to_owned()),
    }
}

fn move_all(src: &Path, dst: &Path) -> io::Result<()> {
    for entry_result in src.read_dir()? {
        let entry = entry_result?;
        let dst_file = dst.join(entry.file_name());
        fs::rename(entry.path(), dst_file)?;
    }
    Ok(())
}

type GenError = Box<std::error::Error>;
type GenResult<T> = Result<T, GenError>;

//fn read_numbers(file: &mut BufRead) -> Result<Vec<i64>, io::Error> {
fn read_numbers(file: &mut BufRead) -> GenResult<Vec<i64>> {
    let mut numbers = vec![];
    for line_result in file.lines() {
        let line = line_result?;
        numbers.push(line.parse()?);
        //error[E0277]: the trait bound `std::io::Error: std::convert::From<std::num::ParseIntError>` is not satisfied
        //  --> src/main.rs:37:22
        //   |
        //37 |         numbers.push(line.parse()?);
        //   |                      ^^^^^^^^^^^^^ the trait `std::convert::From<std::num::ParseIntError>` is not implemented for `std::io::Error`
        //   |
        //   = help: the following implementations were found:
        //             <std::io::Error as std::convert::From<std::ffi::NulError>>
        //             <std::io::Error as std::convert::From<std::io::ErrorKind>>
        //             <std::io::Error as std::convert::From<std::io::IntoInnerError<W>>>
        //   = note: required by `std::convert::From::from`

        // ファイルから一行読むエラーの型と、整数へパースするエラーの型が異なるため
        // コンパイラに怒られている
        // GenResultを定義して、read_numbersの返り値を変更する

    }
    Ok(numbers)
}