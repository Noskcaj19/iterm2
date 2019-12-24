# iTerm2

A Rust crate to allow easy access to the various escape codes in iTerm2.

# Usage

```rust
use iterm2::{AttentionType, Dimension, File};

iterm2::clear_scrollback()?;
iterm2::anchor("https://google.com", "google")?;
iterm2::attention(AttentionType::Firework)?;

File::read("path/to/some/image.png")?
    .height(Dimension::Cells(14))
    .width(Dimension::Percent(100))
    .preserve_aspect_ratio(false)
    .show()?;
```
