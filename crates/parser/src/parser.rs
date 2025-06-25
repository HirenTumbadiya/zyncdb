pub enum Command {
    Put { key: String, value: String },
    Get { key: String },
    Delete { key: String },
    Exit,
    Unknown,
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
            ["PUT", key, value] | ["put", key, value] => Command::Put { key: key.to_string(), value: value.to_string() },
            ["GET", key] | ["get", key] => Command::Get { key: key.to_string() },
            ["DELETE", key] | ["delete", key] => Command::Delete { key: key.to_string() },
            ["EXIT"] | ["exit"] | ["QUIT"] | ["quit"] => Command::Exit,
            _ => Command::Unknown,
        }
    }
}