extern crate base64;

use std::io::Write;

use base64::encode;

pub type TerminalError = std::result::Result<(), std::io::Error>;

pub enum CursorShape {
    Block,
    VerticalBar,
    Underline,
}

pub enum AttentionType {
    Yes,
    No,
    Firework,
}

pub fn anchor<T: Write>(stdout: &mut T, url: &str, display_text: &str) -> TerminalError {
    stdout.write_all(format!("\x1b]8;;{}\x07{}\x1b]8;;\x07\n", url, display_text).as_bytes())
}

pub fn set_cursor_shape<T: Write>(stdout: &mut T, shape: &CursorShape) -> TerminalError {
    use CursorShape::*;
    let shape_val = match *shape {
        Block => 0,
        VerticalBar => 1,
        Underline => 2,
    };
    stdout.write_all(format!("\x1b]1337;CursorShape={}\x07", shape_val).as_bytes())
}


pub fn set_mark<T: Write>(stdout: &mut T) -> TerminalError {
    stdout.write_all(b"\x1b]1337;SetMark\x07")
}

pub fn steal_focus<T: Write>(stdout: &mut T) -> TerminalError {
    stdout.write_all(b"\x1b]1337;StealFocus\x07")
}

pub fn clear_scrollback<T: Write>(stdout: &mut T) -> TerminalError {
    stdout.write_all(b"\x1b]1337;ClearScrollback\x07")
}

pub fn set_current_dir<T: Write>(stdout: &mut T, dir: &str) -> TerminalError {
    stdout.write_all(format!("\x1b]1337;CurrentDir={}\x07", dir).as_bytes())
}

pub fn send_notification<T: Write>(stdout: &mut T, message: &str) -> TerminalError {
    stdout.write_all(format!("\x1b]9;{}\x07", message).as_bytes())
}

// TODO: Add support for the other clipboards
pub fn set_clipboard<T: Write>(stdout: &mut T, text: &str) -> TerminalError {
    stdout.write_all(b"\x1b]1337;CopyToClipboard=\x07")?;
    stdout.write_all(text.as_bytes())?;
    stdout.write_all(b"\n\x1b]1337;EndCopy\x07")
}


pub fn set_tab_colors<T: Write>(stdout: &mut T, red: u8, green: u8, blue: u8) -> TerminalError {
    stdout.write_all(format!("\x1b]6;1;bg;red;brightness;{}\x07", red).as_bytes())?;
    stdout.write_all(format!("\x1b]6;1;bg;green;brightness;{}\x07", green).as_bytes())?;
    stdout.write_all(format!("\x1b]6;1;bg;blue;brightness;{}\x07", blue).as_bytes())
}

pub fn restore_tab_colors<T: Write>(stdout: &mut T) -> TerminalError {
    stdout.write_all(b"\x1b]6;1;bg;*;default\x07")
}

// TODO: Add better parameters
pub fn set_color_palette<T: Write>(stdout: &mut T, colors: &str) -> TerminalError {
    stdout.write_all(format!("\x1b]1337;SetColors={}\x07", colors).as_bytes())
}

pub struct Annotation {
    message: String,
    length: Option<usize>,
    xcoord: Option<usize>,
    ycoord: Option<usize>,
    hidden: bool,
}

impl Annotation {
    pub fn new(message: &str) -> Annotation {
        Annotation {
            message: message.to_owned(),
            length: None,
            xcoord: None,
            ycoord: None,
            hidden: false,
        }
    }
    pub fn length(mut self, length: usize) -> Annotation {
        self.length = Some(length);
        self
    }
    pub fn coords(mut self, x: usize, y: usize) -> Annotation {
        self.xcoord = Some(x);
        self.ycoord = Some(y);
        self
    }
    pub fn hidden(mut self, hide: bool) -> Annotation {
        self.hidden = hide;
        self
    }
    pub fn show<T: Write>(self, stdout: &mut T) -> TerminalError {
        let value = match self {
            Annotation {
                message: msg,
                length: None,
                xcoord: None,
                ycoord: None,
                ..
            } => msg,
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
            } => {
                println!("Message and len, with cords");
                format!("{}|{}|{}|{}", msg, len, x, y)
            }
            _ => panic!("Invalid parameters"), //TODO: Convert to custom error
        };
        let key = if self.hidden {
            "AddHiddenAnnotation"
        } else {
            "AddAnnotation"
        };
        stdout.write_all(format!("\x1b]1337;{}={}\x07", key, value).as_bytes())
    }
}


pub fn cursor_guide<T: Write>(stdout: &mut T, show: bool) -> TerminalError {
    let value = if show { "yes" } else { "no" };
    stdout.write_all(format!("\x1b]1337;HighlightCursorLine={}\x07", value).as_bytes())
}

pub fn attention<T: Write>(stdout: &mut T, kind: &AttentionType) -> TerminalError {
    use AttentionType::*;
    let value = match *kind {
        Yes => "yes",
        No => "no",
        Firework => "fireworks",
    };
    stdout.write_all(format!("\x1b]1337;RequestAttention={}\x07", value).as_bytes())
}

pub fn set_background_image<T: Write>(stdout: &mut T, filename: &str) -> TerminalError {
    let base64_filename = encode(filename.as_bytes());
    stdout.write_all(format!("\x1b]1337;SetBackgroundImageFile={}\x07", base64_filename).as_bytes())
}

//TODO: Implement
#[allow(unused_variables)]
pub fn get_cell_size<T: Write>(stdout: &mut T, filename: &str) -> TerminalError {
    unimplemented!()
}

//TODO: Implement
#[allow(unused_variables)]
pub fn get_terminal_variable<T: Write>(stdout: &mut T, filename: &str) -> TerminalError {
    unimplemented!()
}

pub fn download_image<T: Write>(stdout: &mut T, args: &str, img_data: &[u8]) -> TerminalError {
    stdout.write_all(format!("\x1b]1337;File={}:", args).as_bytes())?;
    stdout.write_all(img_data)?;
    stdout.write_all(b"\x07")
}

pub fn set_touchbar_key_label<T: Write>(stdout: &mut T, key: &str, value: &str) -> TerminalError {
    stdout.write_all(format!("\x1b]1337;SetKeyLabel={}={}\x07", key, value).as_bytes())
}

pub fn push_current_touchbar_label<T: Write>(stdout: &mut T) -> TerminalError {
    stdout.write_all(b"\x1b]1337;PushKeyLabels\x07")
}

pub fn pop_current_touchbar_label<T: Write>(stdout: &mut T) -> TerminalError {
    stdout.write_all(b"\x1b]1337;PopKeyLabels\x07")
}

pub fn push_touchbar_label<T: Write>(stdout: &mut T, label: &str) -> TerminalError {
    stdout.write_all(format!("\x1b1337;PushKeyLabels={}\x07", label).as_bytes())
}

pub fn pop_touchbar_label<T: Write>(stdout: &mut T, label: &str) -> TerminalError {
    stdout.write_all(format!("\x1b1337;PopKeyLabels={}\x07", label).as_bytes())
}

pub fn set_unicode_version<T: Write>(stdout: &mut T, version: u8) -> TerminalError {
    stdout.write_all(format!("\x1b1337;UnicodeVersion={}\x07", version).as_bytes())
}
