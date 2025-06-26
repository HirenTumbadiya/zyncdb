#[test]
fn test_put_and_get() {
    use std::process::{Command, Stdio};

    let mut child = Command::new("cargo")
        .args(&["run", "-p", "cli"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start CLI");

    let stdin = child.stdin.as_mut().unwrap();
    use std::io::Write;
    writeln!(stdin, "put foo bar").unwrap();
    writeln!(stdin, "get foo").unwrap();
    writeln!(stdin, "exit").unwrap();

    let output = child.wait_with_output().unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("ok"));
    assert!(stdout.contains("bar"));
}