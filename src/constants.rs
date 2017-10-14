use sdl2::pixels::Color;

enum_from_primitive! {
    #[derive(Copy)]
    pub enum BackgroundColor {
        Black, White, Aqua, Gray, Yellow
    }
}
impl Clone for BackgroundColor {
    fn clone(&self) -> BackgroundColor { *self }
}

impl BackgroundColor {
    pub fn value(&self) -> Color {
        match *self {
            BackgroundColor::Black => Color::RGB(0, 0, 0),
            BackgroundColor::Aqua => Color::RGB(0, 255, 255),
            BackgroundColor::White => Color::RGB(255, 255, 255),
            BackgroundColor::Gray => Color::RGB(200, 200, 200),
            BackgroundColor::Yellow => Color::RGB(255, 155, 0),
        }
    }
}

pub enum Direction {
    East, South, West, North
}
impl Direction {
    pub fn value(&self) -> i16 {
        match *self {
            Direction::East => 0,
            Direction::South => 90,
            Direction::West => 180,
            Direction::North => 270
        }
    }
}