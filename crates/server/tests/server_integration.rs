#[test]
fn test_server_put_and_get() {
    use std::net::TcpStream;
    use std::io::{Write, BufRead, BufReader};

    let mut stream = TcpStream::connect("127.0.0.1:6379").expect("Connect failed");
    let mut reader = BufReader::new(stream.try_clone().unwrap());

    writeln!(stream, "put foo bar").unwrap();
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    assert!(line.contains("ok"));

    writeln!(stream, "get foo").unwrap();
    line.clear();
    reader.read_line(&mut line).unwrap();
    assert!(line.contains("bar"));
}