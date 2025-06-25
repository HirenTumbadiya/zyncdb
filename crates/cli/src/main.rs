use std::io::{self, Write};
use std::path::PathBuf;

use zyncdb_core::KvStore;
use parser::{SimpleParser, Parser, Command}; // <-- Add parser import

fn main() -> io::Result<()> {
    println!("Welcome to zyncdb 🦀");
    println!("Type 'put <key> <value>', 'get <key>', 'delete <key>', or 'exit'");

    let wal_path = PathBuf::from(".zyncdb.wal");
    let mut store = KvStore::open(&wal_path)?;

    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let parser = SimpleParser; // <-- Create parser

    loop {
        print!("> ");
        stdout.flush()?;

        let mut input = String::new();
        stdin.read_line(&mut input)?;

        let command = parser.parse(&input); // <-- Use parser

        match command {
            Command::Put { key, value } => {
                store.insert(key, value);
                println!("ok");
            }
            Command::Get { key } => match store.get(&key) {
                Some(value) => println!("{}", value),
                None => println!("(key not found)"),
            },
            Command::Delete { key } => {
                if store.delete(&key) {
                    println!("deleted");
                } else {
                    println!("(key not found)");
                }
            }
            Command::Exit => break,
            Command::Unknown => println!("Unknown command. Try: put/get/delete/exit"),
        }
    }

    Ok(())
}
