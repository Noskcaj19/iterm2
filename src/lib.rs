//!
//! # iTerm2
//! A Rust crate to allow easy access to the various escape codes in iTerm2.
//!
//! # Usage
//!
//! ```rust
//! extern crate iterm2;
//! use iterm2::*;
//!
//! clear_scrollback().unwrap();
//! anchor("https://google.com", "google").unwrap();
//! attention(AttentionType::Firework).unwrap();
//! ```
//!

extern crate base64;

use base64::encode;
use std::io::stdout;
use std::io::Write;

pub type TerminalError = Result<(), std::io::Error>;

/// The possible cusor shpaes
#[derive(Debug, Clone, Copy)]
pub enum CursorShape {
    /// A solid vertical block
    Block,
    /// A thin vertical line
    VerticalBar,
    /// A thin horizonal line
    Underline,
}

/// The possible types of attention actions
#[derive(Debug, Clone, Copy)]
pub enum AttentionType {
    /// Start visual display
    Yes,
    /// Stop visual display
    No,
    /// Show fireworks
    Firework,
}

/// Display a clickable link with custom display text
pub fn anchor(url: &str, display_text: &str) -> TerminalError {
    stdout().write_all(format!("\x1b]8;;{}\x07{}\x1b]8;;\x07", url, display_text).as_bytes())
}

/// Set the shape of the cursor
pub fn set_cursor_shape(shape: CursorShape) -> TerminalError {
    use CursorShape::*;
    let shape_val = match shape {
        Block => 0,
        VerticalBar => 1,
        Underline => 2,
    };
    stdout().write_all(format!("\x1b]1337;CursorShape={}\x07", shape_val).as_bytes())
}

/// Set a mark at the current line
pub fn set_mark() -> TerminalError {
    stdout().write_all(b"\x1b]1337;SetMark\x07")
}

/// Attempt to make iTerm the focused application
pub fn steal_focus() -> TerminalError {
    stdout().write_all(b"\x1b]1337;StealFocus\x07")
}

/// Clear the terminals scroll history
pub fn clear_scrollback() -> TerminalError {
    stdout().write_all(b"\x1b]1337;ClearScrollback\x07")
}

/// Sets the terminals current working directory
pub fn set_current_dir(dir: &str) -> TerminalError {
    stdout().write_all(format!("\x1b]1337;CurrentDir={}\x07", dir).as_bytes())
}

/// Send a system wide Growl notification
pub fn send_notification(message: &str) -> TerminalError {
    stdout().write_all(format!("\x1b]9;{}\x07", message).as_bytes())
}

/// Sets the clipboard
// TODO: Add support for the other clipboards
pub fn set_clipboard(text: &str) -> TerminalError {
    stdout().write_all(b"\x1b]1337;CopyToClipboard=\x07")?;
    stdout().write_all(text.as_bytes())?;
    stdout().write_all(b"\n\x1b]1337;EndCopy\x07")
}

/// Sets the tab colors to a custom rgb value
pub fn set_tab_colors(red: u8, green: u8, blue: u8) -> TerminalError {
    stdout().write_all(format!("\x1b]6;1;bg;red;brightness;{}\x07", red).as_bytes())?;
    stdout().write_all(format!("\x1b]6;1;bg;green;brightness;{}\x07", green).as_bytes())?;
    stdout().write_all(format!("\x1b]6;1;bg;blue;brightness;{}\x07", blue).as_bytes())
}

/// Restore the tab colors to defaults
pub fn restore_tab_colors() -> TerminalError {
    stdout().write_all(b"\x1b]6;1;bg;*;default\x07")
}

/// Sets the terminal color palette
///
/// For details on the format, see "Change the color palette" at https://www.iterm2.com/documentation-escape-codes.html
// TODO: Add better parameters
pub fn set_color_palette(colors: &str) -> TerminalError {
    stdout().write_all(format!("\x1b]1337;SetColors={}\x07", colors).as_bytes())
}

/// A builder for terminal annotations
pub struct Annotation {
    message: String,
    length: Option<usize>,
    xcoord: Option<usize>,
    ycoord: Option<usize>,
    hidden: bool,
}

impl Annotation {
    /// Create a new annotation with given text
    pub fn new(message: &str) -> Annotation {
        Annotation {
            message: message.to_owned(),
            length: None,
            xcoord: None,
            ycoord: None,
            hidden: false,
        }
    }

    /// Set the length of the annotation
    pub fn length(&mut self, length: usize) -> &mut Annotation {
        self.length = Some(length);
        self
    }

    /// Set the (x,y) coordinates of the annotation
    pub fn coords(&mut self, x: usize, y: usize) -> &mut Annotation {
        self.xcoord = Some(x);
        self.ycoord = Some(y);
        self
    }

    /// Set the annotation to be hidden
    pub fn hidden(&mut self, hide: bool) -> &mut Annotation {
        self.hidden = hide;
        self
    }

    /// Display the annotation
    pub fn show(&self) -> TerminalError {
        let value = match self {
            Annotation {
                message: msg,
                length: None,
                xcoord: None,
                ycoord: None,
                ..
            } => msg.to_owned(),
            Annotation {
                message: msg,
                length: Some(len),
                xcoord: None,
                ycoord: None,
                ..
            } => format!("{}|{}", len, msg),
            Annotation {
                message: msg,
                length: Some(len),
                xcoord: Some(x),
                ycoord: Some(y),
                ..
            } => format!("{}|{}|{}|{}", msg, len, x, y),
            _ => panic!("Invalid parameters"), //TODO: Convert to custom error
        };
        let key = if self.hidden {
            "AddHiddenAnnotation"
        } else {
            "AddAnnotation"
        };
        stdout().write_all(format!("\x1b]1337;{}={}\x07", key, value).as_bytes())
    }
}

/// Set the visibility of the cursor guide
pub fn cursor_guide(show: bool) -> TerminalError {
    let value = if show { "yes" } else { "no" };
    stdout().write_all(format!("\x1b]1337;HighlightCursorLine={}\x07", value).as_bytes())
}

/// Trigger a dock bounce notification or fireworks
pub fn attention(kind: AttentionType) -> TerminalError {
    use AttentionType::*;
    let value = match kind {
        Yes => "yes",
        No => "no",
        Firework => "fireworks",
    };
    stdout().write_all(format!("\x1b]1337;RequestAttention={}\x07", value).as_bytes())
}

/// Set the terminal background to the image at a path
pub fn set_background_image(filename: &str) -> TerminalError {
    let base64_filename = encode(filename.as_bytes());
    stdout()
        .write_all(format!("\x1b]1337;SetBackgroundImageFile={}\x07", base64_filename).as_bytes())
}

/// Gets the size of a cell in points as a floating point number
///
/// *Not yet implemented*
//TODO: Implement
#[allow(unused_variables)]
pub fn get_cell_size(filename: &str) -> Result<(f32, f32), std::io::Error> {
    unimplemented!()
}

/// Gets the value of a session variable
///
/// *Not yet implemented*
//TODO: Implement
#[allow(unused_variables)]
pub fn get_terminal_variable(filename: &str) -> Result<String, std::io::Error> {
    unimplemented!()
}

/// Show an image inline. Accepts raw image data base64 encoded
///
/// See the [iTerm2 docs](https://www.iterm2.com/documentation-images.html) for more information
pub fn download_image(args: &str, img_data: &[u8]) -> TerminalError {
    stdout().write_all(format!("\x1b]1337;File={}:", args).as_bytes())?;
    stdout().write_all(img_data)?;
    stdout().write_all(b"\x07")
}

/// Configures touchbar key lables
///
/// Seethe [iTerm2 docs](https://www.iterm2.com/documentation-escape-codes.html) for more information
pub fn set_touchbar_key_label(key: &str, value: &str) -> TerminalError {
    stdout().write_all(format!("\x1b]1337;SetKeyLabel={}={}\x07", key, value).as_bytes())
}

/// Push the current key labels
pub fn push_current_touchbar_labels() -> TerminalError {
    stdout().write_all(b"\x1b]1337;PushKeyLabels\x07")
}

/// Pop the current key labels
pub fn pop_current_touchbar_labels() -> TerminalError {
    stdout().write_all(b"\x1b]1337;PopKeyLabels\x07")
}

/// Push a specific touchbar key label by name
pub fn push_touchbar_label(label: &str) -> TerminalError {
    stdout().write_all(format!("\x1b1337;PushKeyLabels={}\x07", label).as_bytes())
}

/// Pop a specific touchbar key label by name
pub fn pop_touchbar_label(label: &str) -> TerminalError {
    stdout().write_all(format!("\x1b1337;PopKeyLabels={}\x07", label).as_bytes())
}

/// Sets the terminals unicode version
pub fn set_unicode_version(version: u8) -> TerminalError {
    stdout().write_all(format!("\x1b1337;UnicodeVersion={}\x07", version).as_bytes())
}
