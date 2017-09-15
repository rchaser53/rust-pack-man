extern crate sdl2;
use sdl2::pixels::Color;

pub enum BackgroundColor {
    black, white
}

impl BackgroundColor {
    pub fn value(&self) -> Color {
        match *self {
            BackgroundColor::black => Color::RGB(0, 0, 0),
            BackgroundColor::white => Color::RGB(255, 255, 255),
        }
    }
}