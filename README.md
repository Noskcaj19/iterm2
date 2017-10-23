# iTerm2

A Rust crate to allow easy access to the various escape codes in iTerm2.

# Usage

```rust
let stdout = io::stdout();
let mut stdout = stdout.lock();

clear_scrollback(&mut stdout).unwrap();
anchor(&mut stdout, "https://google.com", "google").unwrap();
attention(&mut stdout, &AttentionType::Firework).unwrap();
```