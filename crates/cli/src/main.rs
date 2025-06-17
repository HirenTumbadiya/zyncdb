use std::io::{self, Write};
use std::path::PathBuf;

use zyncdb_core::KvStore;

fn main() -> io::Result<()> {
    println!("Welcome to zyncdb 🦀");
    println!("Type 'put <key> <value>', 'get <key>', 'delete <key>', or 'exit'");

    let wal_path = PathBuf::from(".zyncdb.wal");
    let mut store = KvStore::open(&wal_path)?;

    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        print!("> ");
        stdout.flush()?;

        let mut input = String::new();
        stdin.read_line(&mut input)?;

        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        let parts: Vec<&str> = input.splitn(3, ' ').collect();
        match parts.as_slice() {
            ["exit"] | ["quit"] => break,

            ["put", key, value] => {
                store.insert(key.to_string(), value.to_string());
                println!("ok");
            }

            ["get", key] => match store.get(key) {
                Some(value) => println!("{}", value),
                None => println!("(key not found)"),
            },

            ["delete", key] => {
                if store.delete(key) {
                    println!("deleted");
                } else {
                    println!("(key not found)");
                }
            }

            _ => {
                println!("Unknown command. Try: put/get/delete/exit");
            }
        }
    }

    Ok(())
}
