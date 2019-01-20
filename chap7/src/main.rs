fn main() {
    let hometown = "Tokyo1";
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