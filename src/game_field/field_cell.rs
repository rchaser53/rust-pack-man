use num::FromPrimitive;
use sdl2::{render, video};

use game_field::field::{CELL_WIDTH, CELL_HEIGHT};
use game_field::cell_feature::{CellFeature};

#[derive(Copy)]
pub struct CellStatus {
    pub width: u32,
    pub height: u32,
    pub x: i32,
    pub y: i32,
    pub cell_type: CellType,
    pub exist_item: bool
}
impl CellStatus {
    pub fn new(cell_type: i16, row_index: usize, cell_index: usize) -> CellStatus {
        CellStatus {
            x: (cell_index * CELL_WIDTH as usize) as i32,
            y: (row_index * CELL_HEIGHT as usize) as i32,
            width: CELL_WIDTH as u32,
            height: CELL_HEIGHT as u32,
            cell_type: CellType::from_i16(cell_type).unwrap(),
            exist_item: CellStatus::get_initial_exist_item(cell_type)
        }
    }

    pub fn get_initial_exist_item(cell_type: i16) -> bool {
        CellType::from_i16(cell_type).unwrap() == CellType::Item
    }
}
impl Clone for CellStatus {
    fn clone(&self) -> CellStatus { *self }
}

enum_from_primitive! {
    #[derive(Copy, PartialEq)]
    pub enum CellType {
        Normal, Block, Damage, Wall, Item
    }
}
impl Clone for CellType {
    fn clone(&self) -> CellType { *self }
}

pub struct FieldCell {
    pub status: CellStatus
}

impl FieldCell {
    pub fn new(cell_type: i16, row_index: usize, cell_index: usize) -> FieldCell {
        FieldCell {
            status: CellStatus::new(cell_type, row_index, cell_index)
        }
    }

    pub fn draw(&self, renderer: &mut render::Canvas<video::Window>) {
        CellFeature::draw(&self.status, renderer);
    }
}