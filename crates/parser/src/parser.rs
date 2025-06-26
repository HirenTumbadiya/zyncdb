pub enum Command {
    Put { key: String, value: String },
    Get { key: String },
    Delete { key: String },
    Insert { key: String, value: String },
    Select { key: String },
    Remove { key: String },
    Snapshot,
    List,
    Exit,
    Unknown,
    Ttl { key: String, seconds: u64 },
    Batch(Vec<Command>),
}


pub trait Parser {
    fn parse(&self, input: &str) -> Command;
}

pub struct SimpleParser;

impl Parser for SimpleParser {
    fn parse(&self, input: &str) -> Command {
        let input = input.trim();
        let tokens: Vec<&str> = input.split_whitespace().collect();

        match tokens.as_slice() {
            ["INSERT", key, rest @ ..] if !rest.is_empty() => Command::Insert {
                key: key.to_string(),
                value: rest.join(" "),
            },
            ["SELECT", key] => Command::Select {
                key: key.to_string(),
            },
            ["REMOVE", key] => Command::Remove {
                key: key.to_string(),
            },
            ["PUT", key, rest @ ..] | ["put", key, rest @ ..] if !rest.is_empty() => Command::Put {
                key: key.to_string(),
                value: rest.join(" "),
            },
            ["GET", key] | ["get", key] => Command::Get {
                key: key.to_string(),
            },
            ["DELETE", key] | ["delete", key] => Command::Delete {
                key: key.to_string(),
            },
            ["SNAPSHOT"] | ["snapshot"] => Command::Snapshot,
            ["LIST"] | ["list"] | ["KEYS"] | ["keys"] => Command::List,
            ["EXIT"] | ["exit"] | ["QUIT"] | ["quit"] => Command::Exit,
            ["TTL", key, secs] if secs.parse::<u64>().is_ok() => Command::Ttl {
                key: key.to_string(),
                seconds: secs.parse().unwrap(),
            },
            ["BATCH", rest @ ..] => {
                // Parse rest as multiple put/delete commands
                // For simplicity, only support "put key value" for now
                let mut cmds = Vec::new();
                let mut i = 0;
                while i < rest.len() {
                    match rest.get(i) {
                        Some(&"put") if i + 2 < rest.len() => {
                            cmds.push(Command::Put {
                                key: rest[i + 1].to_string(),
                                value: rest[i + 2].to_string(),
                            });
                            i += 3;
                        }
                        _ => break,
                    }
                }
                Command::Batch(cmds)
            }
            _ => Command::Unknown,
        }
    }
}
