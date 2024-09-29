use core::fmt;

use ratatui::style::Color;

pub const UNDEFINED_POSITION: u8 = u8::MAX;
pub const WHITE: Color = Color::Rgb(160, 160, 160);
pub const BLACK: Color = Color::Rgb(128, 95, 69);

pub const TITLE: &str = r"
██╗    ██╗██╗████████╗██╗   ██╗██╗
██║    ██║██║╚══██╔══╝██║   ██║██║
██║ █╗ ██║██║   ██║   ██║   ██║██║
██║███╗██║██║   ██║   ██║   ██║██║
╚███╔███╔╝██║   ██║   ╚██████╔╝██║
 ╚══╝╚══╝ ╚═╝   ╚═╝    ╚═════╝ ╚═╝
";

pub enum DisplayMode {
    DEFAULT,
    ASCII,
}

impl fmt::Display for DisplayMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisplayMode::ASCII => write!(f, "ASCII"),
            DisplayMode::DEFAULT => write!(f, "DEFAULT"),
        }
    }
}
