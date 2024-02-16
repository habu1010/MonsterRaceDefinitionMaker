use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize,
)]
pub enum Color {
    #[default]
    Black,
    White,
    Gray,
    Orange,
    Red,
    Green,
    Blue,
    Brown,
    DarkGray,
    LightGray,
    Violet,
    Yellow,
    LightRed,
    LightGreen,
    LightBlue,
    LightBrown,
}

pub struct ColorSymbol(char);

use Color::*;

pub const COLORS: [Color; 16] = [
    Black, White, Gray, Orange, Red, Green, Blue, Brown, DarkGray, LightGray, Violet, Yellow,
    LightRed, LightGreen, LightBlue, LightBrown,
];

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Black => write!(f, "黒"),
            White => write!(f, "白"),
            Gray => write!(f, "灰"),
            Orange => write!(f, "橙"),
            Red => write!(f, "赤"),
            Green => write!(f, "緑"),
            Blue => write!(f, "青"),
            Brown => write!(f, "茶"),
            DarkGray => write!(f, "暗い灰"),
            LightGray => write!(f, "明るい灰"),
            Violet => write!(f, "バイオレット"),
            Yellow => write!(f, "黄"),
            LightRed => write!(f, "明るい赤"),
            LightGreen => write!(f, "明るい緑"),
            LightBlue => write!(f, "明るい青"),
            LightBrown => write!(f, "明るい茶"),
        }
    }
}

impl fmt::Display for ColorSymbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Color> for ColorSymbol {
    fn from(color: Color) -> Self {
        let symbol = match color {
            Black => 'D',
            White => 'w',
            Gray => 's',
            Orange => 'o',
            Red => 'r',
            Green => 'g',
            Blue => 'b',
            Brown => 'u',
            DarkGray => 'd',
            LightGray => 'W',
            Violet => 'v',
            Yellow => 'y',
            LightRed => 'R',
            LightGreen => 'G',
            LightBlue => 'B',
            LightBrown => 'U',
        };
        ColorSymbol(symbol)
    }
}

impl From<Color> for egui::Color32 {
    fn from(color: Color) -> Self {
        match color {
            Black => egui::Color32::BLACK,
            White => egui::Color32::WHITE,
            Gray => egui::Color32::from_rgb(0x80, 0x80, 0x80),
            Orange => egui::Color32::from_rgb(0xff, 0x80, 0x00),
            Red => egui::Color32::from_rgb(0xc0, 0x00, 0x00),
            Green => egui::Color32::from_rgb(0x00, 0x80, 0x40),
            Blue => egui::Color32::from_rgb(0x00, 0x80, 0xff),
            Brown => egui::Color32::from_rgb(0x80, 0x40, 0x00),
            DarkGray => egui::Color32::from_rgb(0x40, 0x40, 0x40),
            LightGray => egui::Color32::from_rgb(0xc0, 0xc0, 0xc0),
            Violet => egui::Color32::from_rgb(0xff, 0x00, 0xff),
            Yellow => egui::Color32::from_rgb(0xff, 0xff, 0x00),
            LightRed => egui::Color32::from_rgb(0xff, 0x00, 0x00),
            LightGreen => egui::Color32::from_rgb(0x00, 0xff, 0x00),
            LightBlue => egui::Color32::from_rgb(0x00, 0xff, 0xff),
            LightBrown => egui::Color32::from_rgb(0xc0, 0x80, 0x40),
        }
    }
}
