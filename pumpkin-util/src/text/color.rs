use colored::{ColoredString, Colorize};
use serde::{Deserialize, Deserializer, Serialize};

/// Text color
#[derive(Default, Debug, Clone, Copy, Serialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum Color {
    /// The default color for the text will be used, which varies by context
    /// (in some cases, it's white; in others, it's black; in still others, it
    /// is a shade of gray that isn't normally used on text).
    #[default]
    Reset,
    /// RGB Color
    Rgb(RGBColor),
    /// One of the 16 named Minecraft colors
    Named(NamedColor),
}

pub fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (u8, u8, u8) {
    let c = v * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = v - c;
    let (r, g, b) = match (h as i32 / 60) % 6 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };
    (
        ((r + m) * 255.0) as u8,
        ((g + m) * 255.0) as u8,
        ((b + m) * 255.0) as u8,
    )
}

impl<'de> Deserialize<'de> for Color {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;

        if s == "reset" {
            Ok(Color::Reset)
        } else if let Some(hex) = s.strip_prefix('#') {
            if s.len() != 7 {
                return Err(serde::de::Error::custom(
                    "Hex color must be in the format '#RRGGBB'",
                ));
            }

            let r = u8::from_str_radix(&hex[0..2], 16)
                .map_err(|_| serde::de::Error::custom("Invalid red component in hex color"))?;
            let g = u8::from_str_radix(&hex[2..4], 16)
                .map_err(|_| serde::de::Error::custom("Invalid green component in hex color"))?;
            let b = u8::from_str_radix(&hex[4..6], 16)
                .map_err(|_| serde::de::Error::custom("Invalid blue component in hex color"))?;

            Ok(Color::Rgb(RGBColor::new(r, g, b)))
        } else {
            Ok(Color::Named(NamedColor::try_from(s.as_str()).map_err(
                |_| serde::de::Error::custom("Invalid named color"),
            )?))
        }
    }
}

impl Color {
    pub fn console_color(&self, text: &str) -> ColoredString {
        match self {
            Color::Reset => text.clear(),
            Color::Named(color) => match color {
                NamedColor::Black => text.black(),
                NamedColor::DarkBlue => text.blue(),
                NamedColor::DarkGreen => text.green(),
                NamedColor::DarkAqua => text.cyan(),
                NamedColor::DarkRed => text.red(),
                NamedColor::DarkPurple => text.purple(),
                NamedColor::Gold => text.yellow(),
                NamedColor::Gray => text.bright_black(),
                NamedColor::DarkGray => text.bright_black(), // ?
                NamedColor::Blue => text.bright_blue(),
                NamedColor::Green => text.bright_green(),
                NamedColor::Aqua => text.cyan(),
                NamedColor::Red => text.red(),
                NamedColor::LightPurple => text.bright_purple(),
                NamedColor::Yellow => text.bright_yellow(),
                NamedColor::White => text.white(),
            },
            // TODO: Check if terminal supports true color
            Color::Rgb(color) => text.truecolor(color.red, color.green, color.blue),
        }
    }
}

#[derive(Debug, Deserialize, Clone, Copy, Eq, Hash, PartialEq)]
pub struct RGBColor {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl RGBColor {
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        RGBColor { red, green, blue }
    }
}

impl Serialize for RGBColor {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&format!(
            "#{:02X}{:02X}{:02X}",
            self.red, self.green, self.blue
        ))
    }
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq, Deserialize)]
pub struct ARGBColor {
    alpha: u8,
    red: u8,
    green: u8,
    blue: u8,
}

impl ARGBColor {
    pub fn new(alpha: u8, red: u8, green: u8, blue: u8) -> Self {
        ARGBColor {
            alpha,
            red,
            green,
            blue,
        }
    }
}

impl Serialize for ARGBColor {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_bytes([self.alpha, self.red, self.green, self.blue].as_ref())
    }
}

/// Named Minecraft color
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NamedColor {
    Black = 0,
    DarkBlue,
    DarkGreen,
    DarkAqua,
    DarkRed,
    DarkPurple,
    Gold,
    Gray,
    DarkGray,
    Blue,
    Green,
    Aqua,
    Red,
    LightPurple,
    Yellow,
    White,
}

impl NamedColor {
    pub fn to_rgb(&self) -> RGBColor {
        match self {
            NamedColor::Black => RGBColor::new(0, 0, 0),
            NamedColor::DarkBlue => RGBColor::new(0, 0, 170),
            NamedColor::DarkGreen => RGBColor::new(0, 170, 0),
            NamedColor::DarkAqua => RGBColor::new(0, 170, 170),
            NamedColor::DarkRed => RGBColor::new(170, 0, 0),
            NamedColor::DarkPurple => RGBColor::new(170, 0, 170),
            NamedColor::Gold => RGBColor::new(255, 170, 0),
            NamedColor::Gray => RGBColor::new(170, 170, 170),
            NamedColor::DarkGray => RGBColor::new(85, 85, 85),
            NamedColor::Blue => RGBColor::new(85, 85, 255),
            NamedColor::Green => RGBColor::new(85, 255, 85),
            NamedColor::Aqua => RGBColor::new(85, 255, 255),
            NamedColor::Red => RGBColor::new(255, 85, 85),
            NamedColor::LightPurple => RGBColor::new(255, 85, 255),
            NamedColor::Yellow => RGBColor::new(255, 255, 85),
            NamedColor::White => RGBColor::new(255, 255, 255),
        }
    }
}

impl TryFrom<&str> for NamedColor {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "black" => Ok(NamedColor::Black),
            "dark_blue" => Ok(NamedColor::DarkBlue),
            "dark_green" => Ok(NamedColor::DarkGreen),
            "dark_aqua" => Ok(NamedColor::DarkAqua),
            "dark_red" => Ok(NamedColor::DarkRed),
            "dark_purple" => Ok(NamedColor::DarkPurple),
            "gold" => Ok(NamedColor::Gold),
            "gray" => Ok(NamedColor::Gray),
            "dark_gray" => Ok(NamedColor::DarkGray),
            "blue" => Ok(NamedColor::Blue),
            "green" => Ok(NamedColor::Green),
            "aqua" => Ok(NamedColor::Aqua),
            "red" => Ok(NamedColor::Red),
            "light_purple" => Ok(NamedColor::LightPurple),
            "yellow" => Ok(NamedColor::Yellow),
            "white" => Ok(NamedColor::White),
            _ => Err(()),
        }
    }
}
