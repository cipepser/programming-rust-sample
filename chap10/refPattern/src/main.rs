#[derive(Debug)]
pub struct Account {
    id: u32,
    name: String,
    language: String,
}

fn main() {
    let account = Account {
        id: 1,
        name: "Alice".to_string(),
        language: "English".to_string(),
    };

    match account {
        Account { ref name, ref language, .. } => {
            println!("{}: {}", name, language);
            let msg = "Hello ".to_string() + name;
            println!("{}", msg);
        }
    }
}
