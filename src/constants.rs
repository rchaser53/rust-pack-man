use sdl2::pixels::Color;
use std::collections::HashMap;

lazy_static! {
  pub static ref FILE_PATHS: HashMap<&'static str, &'static str> = {
    let mut map = HashMap::new();
    map.insert("BGM_PATH", "assets/musics/loop1.wav");
    map.insert("HIT_EFFECT_PATH", "assets/musics/sine.wav");

    #[cfg(target_os = "emscripten")]
    map.insert("SQUARE_MAP_PATH", "sample_map1.txt");
    #[cfg(not(target_os = "emscripten"))]
    map.insert("SQUARE_MAP_PATH", "assets/maps/sample_map1.txt");

    map
  };
}

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

#[derive(Copy)]
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

    pub fn opposite(&self) -> Direction {
        match *self {
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
            Direction::North => Direction::South
        }
    }

    pub fn clockwise(&self) -> Direction {
        match *self {
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
            Direction::North => Direction::East
        }
    }
}
impl Clone for Direction {
    fn clone(&self) -> Direction { *self }
}

#[derive(Copy, Debug, PartialEq)]
pub enum CellType {
    Normal, Block, Damage, Wall, Item
}
impl Clone for CellType {
    fn clone(&self) -> CellType { *self }
}

pub const CELL_WIDTH: i16 = 30;
pub const CELL_HEIGHT: i16 = 30;