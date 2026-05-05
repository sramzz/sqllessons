use std::env;
use std::io::{self, Read, Write};

use base64::engine::general_purpose::STANDARD;
use base64::Engine;

const USAGE: &str = "Usage: base64tool <encode|decode> [text]";

fn encode_bytes(input: &[u8]) -> String {
    STANDARD.encode(input)
}

fn decode_base64(input: &str) -> Result<Vec<u8>, String> {
    let normalized: String = input.chars().filter(|c| !c.is_ascii_whitespace()).collect();
    STANDARD
        .decode(normalized)
        .map_err(|error| error.to_string())
}

fn read_stdin() -> io::Result<Vec<u8>> {
    let mut input = Vec::new();
    io::stdin().read_to_end(&mut input)?;
    Ok(input)
}

fn print_usage_and_exit() -> ! {
    eprintln!("{USAGE}");
    std::process::exit(1);
}

fn run() -> Result<(), String> {
    let args: Vec<String> = env::args().skip(1).collect();

    let Some(command) = args.first() else {
        print_usage_and_exit();
    };

    if args.len() > 2 {
        print_usage_and_exit();
    }

    match command.as_str() {
        "encode" => {
            let input = match args.get(1) {
                Some(text) => text.as_bytes().to_vec(),
                None => read_stdin().map_err(|error| format!("input error: {error}"))?,
            };
            println!("{}", encode_bytes(&input));
        }
        "decode" => {
            let input = match args.get(1) {
                Some(text) => text.clone(),
                None => String::from_utf8(
                    read_stdin().map_err(|error| format!("input error: {error}"))?,
                )
                .map_err(|error| format!("decode error: input is not valid UTF-8: {error}"))?,
            };
            let decoded =
                decode_base64(&input).map_err(|error| format!("decode error: {error}"))?;
            io::stdout()
                .write_all(&decoded)
                .map_err(|error| format!("output error: {error}"))?;
        }
        _ => print_usage_and_exit(),
    }

    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::{decode_base64, encode_bytes};

    #[test]
    fn encodes_hello() {
        assert_eq!(encode_bytes(b"hello"), "aGVsbG8=");
    }

    #[test]
    fn decodes_hello() {
        assert_eq!(decode_base64("aGVsbG8=").unwrap(), b"hello");
    }

    #[test]
    fn encodes_empty_input() {
        assert_eq!(encode_bytes(b""), "");
    }

    #[test]
    fn decodes_empty_input() {
        assert_eq!(decode_base64("").unwrap(), b"");
    }

    #[test]
    fn rejects_malformed_base64() {
        assert!(decode_base64("not valid base64").is_err());
    }
}
