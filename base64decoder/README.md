# base64tool

A small local Base64 encoder and decoder for macOS, written in Rust.

It uses the Rust `base64` crate at build time. After you compile it, the `base64tool` binary runs locally and does not make network calls.

## Build

From this directory:

```bash
cargo build --release
```

The compiled binary will be here:

```bash
target/release/base64tool
```

## Use From This Folder

Encode text:

```bash
./target/release/base64tool encode "hello"
```

Output:

```text
aGVsbG8=
```

Decode text:

```bash
./target/release/base64tool decode "aGVsbG8="
```

Output:

```text
hello
```

You can also pipe input through stdin:

```bash
printf "hello" | ./target/release/base64tool encode
printf "aGVsbG8=" | ./target/release/base64tool decode
```

Use `printf` when you do not want to include a newline. `echo hello` includes a newline, so it encodes to `aGVsbG8K`.

## Install For Your User

This keeps the binary in your home folder and does not require admin permissions.

Create a local bin folder if you do not already have one:

```bash
mkdir -p "$HOME/.local/bin"
```

Copy the compiled binary there:

```bash
cp target/release/base64tool "$HOME/.local/bin/base64tool"
```

Make sure it is executable:

```bash
chmod +x "$HOME/.local/bin/base64tool"
```

Add this line to your shell config if it is not already there:

```bash
echo 'export PATH="$HOME/.local/bin:$PATH"' >> "$HOME/.zshrc"
```

Reload your shell:

```bash
source "$HOME/.zshrc"
```

Now you can run it from anywhere:

```bash
base64tool encode "hello"
base64tool decode "aGVsbG8="
```

## Install System-Wide

This puts the binary in `/usr/local/bin`, which is commonly already in your `PATH` on macOS. It may ask for your password.

```bash
sudo cp target/release/base64tool /usr/local/bin/base64tool
sudo chmod +x /usr/local/bin/base64tool
```

Then run it from anywhere:

```bash
base64tool encode "hello"
base64tool decode "aGVsbG8="
```

## Verify Install

Check that your shell can find it:

```bash
which base64tool
```

Check that it works:

```bash
base64tool encode "hello"
base64tool decode "aGVsbG8="
```

## Usage

```text
Usage: base64tool <encode|decode> [text]
```

- `base64tool encode [text]` encodes the text argument, or stdin if no text is given.
- `base64tool decode [text]` decodes the text argument, or stdin if no text is given.
- Decoded output is written as raw bytes, so it can be printed as text or piped to a file.

## Examples

Encode and save to a file:

```bash
base64tool encode "hello" > encoded.txt
```

Decode from a file:

```bash
base64tool decode < encoded.txt
```

Decode and save binary output:

```bash
base64tool decode < encoded.txt > output.bin
```
