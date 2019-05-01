extern crate reqwest;

use std::net::TcpListener;
use std::io::{self, Write};
use std::error::Error;
use std::thread::spawn;

fn echo_main(addr: &str) -> io::Result<()> {
    let listener = TcpListener::bind(addr)?;
    println!("listening on {}", addr);
    loop {
        let (mut stream, addr) = listener.accept()?;
        println!("connection received from {}", addr);

        let mut write_stream = stream.try_clone()?;
        spawn(move || {
            io::copy(&mut stream, &mut write_stream)
                .expect("error in client thread: ");
            println!("connection closed");
        });
    }
}

fn http_get_main(url: &str) -> Result<(), Box<Error>> {
    let mut response = reqwest::get(url)?;
    if !response.status().is_success() {
        Err(format!("{}", response.status()))?;
    }

    let stdout = io::stdout();
    io::copy(&mut response, &mut stdout.lock())?;

    Ok(())
}

fn main() {
//    echo_main("127.0.0.1:17007").expect("error: ");
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        writeln!(io::stderr(), "usage: http-get URL").unwrap();
        return;
    }

    if let Err(err) = http_get_main(&args[1]) {
        writeln!(io::stderr(), "error: {}", err).unwrap();
    }
}
