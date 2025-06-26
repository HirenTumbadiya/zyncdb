use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpStream;

fn main() -> io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:6379")?;
    let mut reader = BufReader::new(stream.try_clone()?);

    let mut welcome = String::new();
    reader.read_line(&mut welcome)?; // read welcome message
    println!("{}", welcome.trim());

    let stdin = io::stdin();
    let mut input = String::new();

    loop {
        print!("zyncdb> ");
        io::stdout().flush()?;

        input.clear();
        stdin.read_line(&mut input)?;
        if input.trim().is_empty() {
            continue;
        }

        stream.write_all(input.as_bytes())?;
        stream.flush()?;

        let mut response = String::new();
        reader.read_line(&mut response)?;
        println!("{}", response.trim());

        if input.trim() == "exit" {
            break;
        }
    }

    Ok(())
}
