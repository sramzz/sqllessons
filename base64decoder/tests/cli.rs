use std::io::Write;
use std::process::{Command, Stdio};

fn run_with_args(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_base64tool"))
        .args(args)
        .output()
        .expect("failed to run base64tool")
}

fn run_with_stdin(args: &[&str], stdin: &str) -> std::process::Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_base64tool"))
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to spawn base64tool");

    child
        .stdin
        .as_mut()
        .expect("failed to open stdin")
        .write_all(stdin.as_bytes())
        .expect("failed to write stdin");

    child
        .wait_with_output()
        .expect("failed to wait for base64tool")
}

#[test]
fn encodes_argument() {
    let output = run_with_args(&["encode", "hello"]);

    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout), "aGVsbG8=\n");
    assert!(output.stderr.is_empty());
}

#[test]
fn decodes_argument() {
    let output = run_with_args(&["decode", "aGVsbG8="]);

    assert!(output.status.success());
    assert_eq!(output.stdout, b"hello");
    assert!(output.stderr.is_empty());
}

#[test]
fn encodes_empty_argument() {
    let output = run_with_args(&["encode", ""]);

    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout), "\n");
    assert!(output.stderr.is_empty());
}

#[test]
fn decodes_empty_argument() {
    let output = run_with_args(&["decode", ""]);

    assert!(output.status.success());
    assert!(output.stdout.is_empty());
    assert!(output.stderr.is_empty());
}

#[test]
fn encodes_stdin() {
    let output = run_with_stdin(&["encode"], "hello");

    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout), "aGVsbG8=\n");
    assert!(output.stderr.is_empty());
}

#[test]
fn decodes_stdin() {
    let output = run_with_stdin(&["decode"], "aGVsbG8=");

    assert!(output.status.success());
    assert_eq!(output.stdout, b"hello");
    assert!(output.stderr.is_empty());
}

#[test]
fn rejects_malformed_base64() {
    let output = run_with_args(&["decode", "not valid base64"]);

    assert!(!output.status.success());
    assert!(output.stdout.is_empty());
    assert!(String::from_utf8_lossy(&output.stderr).contains("decode error:"));
}

#[test]
fn rejects_unknown_subcommand() {
    let output = run_with_args(&["rot13", "hello"]);

    assert!(!output.status.success());
    assert!(output.stdout.is_empty());
    assert!(String::from_utf8_lossy(&output.stderr)
        .contains("Usage: base64tool <encode|decode> [text]"));
}

#[test]
fn rejects_extra_arguments() {
    let output = run_with_args(&["encode", "hello", "extra"]);

    assert!(!output.status.success());
    assert!(output.stdout.is_empty());
    assert!(String::from_utf8_lossy(&output.stderr)
        .contains("Usage: base64tool <encode|decode> [text]"));
}
