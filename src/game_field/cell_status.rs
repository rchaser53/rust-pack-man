use sdl2::{pixels};

use game_field::field::{CELL_WIDTH, CELL_HEIGHT};

pub struct CellStatus {
    pub width: u32,
    pub height: u32,
    pub x: i32,
    pub y: i32,
    pub cell_type: i32,
    pub background_color: pixels::Color,
    pub exist_item: bool
}

impl CellStatus {
    pub fn new(background_color: pixels::Color, row_index: usize, cell_index: usize) -> CellStatus {
        CellStatus {
            x: (cell_index * CELL_WIDTH as usize) as i32,
            y: (row_index * CELL_HEIGHT as usize) as i32,
            width: CELL_WIDTH as u32,
            height: CELL_HEIGHT as u32,
            background_color: background_color,
            cell_type: 1,
            exist_item: false
        }
    }
}

// pub fn get_initial_exist_item(cell_type: i16) -> bool {
//     CellType::from_i16(cell_type).unwrap() == CellType::Item
// }