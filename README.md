# iTerm2

A Rust crate to allow easy access to the various escape codes in iTerm2.

# Usage

```rust
extern crate iterm2;
use iterm2::*;

clear_scrollback().unwrap();
anchor("https://google.com", "google").unwrap();
attention(AttentionType::Firework).unwrap();
```
