use std::net::{TcpListener, TcpStream};
use std::io::{BufRead, BufReader, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use std::path::PathBuf;
use zyncdb_core::KvStore;
use parser::{SimpleParser, Parser, Command};

fn handle_client(stream: TcpStream, store: Arc<Mutex<KvStore>>) {
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    let mut writer = stream;
    let parser = SimpleParser;

    loop {
        let mut input = String::new();
        if reader.read_line(&mut input).is_err() {
            break;
        }
        let command = parser.parse(&input);
        let mut store = store.lock().unwrap();
        let response = match command {
            Command::Put { key, value } => {
                store.insert(key, value);
                "ok\n".to_string()
            }
            Command::Get { key } => match store.get(&key) {
                Some(value) => format!("{}\n", value),
                None => "(key not found)\n".to_string(),
            },
            Command::Delete { key } => {
                if store.delete(&key) {
                    "deleted\n".to_string()
                } else {
                    "(key not found)\n".to_string()
                }
            }
            Command::Exit => break,
            _ => "Unknown command\n".to_string(),
        };
        let _ = writer.write_all(response.as_bytes());
    }
}

fn main() -> std::io::Result<()> {
    let wal_path = PathBuf::from(".zyncdb.wal");
    let store = Arc::new(Mutex::new(KvStore::open(&wal_path)?));
    let listener = TcpListener::bind("127.0.0.1:6379")?;
    println!("Server listening on 127.0.0.1:6379");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let store = Arc::clone(&store);
                thread::spawn(move || {
                    handle_client(stream, store);
                });
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }
    Ok(())
}