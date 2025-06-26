use std::io::{self, Write};
use std::path::PathBuf;

use parser::{Command, Parser, SimpleParser};
use zyncdb_core::KvStore;

fn main() -> std::io::Result<()> {
    env_logger::init();

    println!("Welcome to zyncdb 🦀");
    println!(
        "Type 'put <key> <value>', 'get <key>', 'delete <key>', or 'exit' to quit. Type 'snapshot' to create a snapshot and compact the database, or 'list' to list all keys."
    );

    let wal_path = PathBuf::from(".zyncdb.wal");
    let mut store = KvStore::open(&wal_path)?;

    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let parser = SimpleParser;

    loop {
        print!("> ");
        stdout.flush()?;

        let mut input = String::new();
        stdin.read_line(&mut input)?;

        let command = parser.parse(&input);

        match command {
            Command::Put { key, value } | Command::Insert { key, value } => {
                if !is_valid_key(&key) {
                    println!("Error: Invalid key '{}'. Keys must not be empty, longer than 255 chars, or contain '|'.", key);
                    continue;
                }
                if value.is_empty() {
                    println!("Error: Value for key '{}' cannot be empty.", key);
                    continue;
                }
                store.insert(key, value);
                println!("ok");
            }
            Command::Get { key } | Command::Select { key } => match store.get(&key) {
                Some(value) => println!("{}", value),
                None => println!("(key not found)"),
            },
            Command::Delete { key } | Command::Remove { key } => {
                if store.delete(&key) {
                    println!("deleted");
                } else {
                    println!("(key not found)");
                }
            }
            Command::Snapshot => {
                let snapshot_path = PathBuf::from(".zyncdb.snapshot");
                let wal_path = PathBuf::from(".zyncdb.wal");
                match store.snapshot_and_compact(&snapshot_path, &wal_path) {
                    Ok(_) => println!("Snapshot and compaction complete."),
                    Err(e) => println!("Snapshot error: {}", e),
                }
            }
            Command::List => {
                for (k, v) in store.iter() {
                    println!("{} = {}", k, v);
                }
            }
            Command::Batch(cmds) => {
                for cmd in cmds {
                    match cmd {
                        Command::Put { key, value } => {
                            store.insert(key, value);
                        }
                        // Add more as needed
                        _ => {}
                    }
                }
                println!("Batch executed");
            }
            Command::Ttl { key, seconds } => {
                store.set_ttl(&key, seconds);
                println!("TTL set for '{}' to {} seconds", key, seconds);
            }
            Command::Exit => break,
            Command::Help => {
                println!("Available commands:");
                println!("  put <key> <value>      - Insert or update a key");
                println!("  get <key>              - Get value for a key");
                println!("  delete <key>           - Delete a key");
                println!("  insert <key> <value>   - SQL-like insert");
                println!("  select <key>           - SQL-like select");
                println!("  remove <key>           - SQL-like remove");
                println!("  ttl <key> <seconds>    - Set time-to-live for a key");
                println!("  batch ...              - Batch operations");
                println!("  snapshot               - Create snapshot and compact WAL");
                println!("  list                   - List all keys/values");
                println!("  help                   - Show this help message");
                println!("  exit                   - Exit the CLI");
            }
            Command::Unknown => {
                println!("Error: Unknown or malformed command. Type 'help' to see available commands.");
            }
        }
    }

    Ok(())
}

fn is_valid_key(key: &str) -> bool {
    !key.is_empty() && key.len() < 256 && !key.contains('|')
}
