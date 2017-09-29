use sdl2::pixels::Color;

pub enum BackgroundColor {
    Black, White, Aqua
}
impl BackgroundColor {
    pub fn value(&self) -> Color {
        match *self {
            BackgroundColor::Black => Color::RGB(0, 0, 0),
            BackgroundColor::Aqua => Color::RGB(0, 255, 255),
            BackgroundColor::White => Color::RGB(255, 255, 255),
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