extern crate sdl2;
use sdl2::pixels::Color;

pub enum BackgroundColor {
    Black, White
}

impl BackgroundColor {
    pub fn value(&self) -> Color {
        match *self {
            BackgroundColor::Black => Color::RGB(0, 0, 0),
            BackgroundColor::White => Color::RGB(255, 255, 255),
        }
    }
}