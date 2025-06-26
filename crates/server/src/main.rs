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

    let _ = writer.write_all("Welcome to zyncdb 🦀\nType 'help' for commands.\n".as_bytes());
    let _ = writer.flush();
    println!("Client connected success");

    loop {
        println!("Client connected");
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
            Command::Help => {
                "Available commands:\n\
                put <key> <value>\n\
                get <key>\n\
                delete <key>\n\
                insert <key> <value>\n\
                select <key>\n\
                remove <key>\n\
                ttl <key> <seconds>\n\
                batch ...\n\
                snapshot\n\
                list\n\
                help\n\
                exit\n".to_string()
            }
            Command::Exit => break,
            _ => "Unknown command\n".to_string(),
        };
        let _ = writer.write_all(response.as_bytes());
    }
}

fn main() -> std::io::Result<()> {
    env_logger::init();
    let wal_path = PathBuf::from(".zyncdb.wal");
    let store = Arc::new(Mutex::new(KvStore::open(&wal_path)?));
    let listener = TcpListener::bind("127.0.0.1:6379")?;
    log::info!("Server listening on 127.0.0.1:6379");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let store = Arc::clone(&store);
                thread::spawn(move || {
                    log::debug!("Client connected: {:?}", stream.peer_addr());
                    handle_client(stream, store);
                });
            }
            Err(e) => {
                log::error!("Connection failed: {}", e);
            }
        }
    }
    Ok(())
}