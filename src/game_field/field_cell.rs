use num::FromPrimitive;
use sdl2::{render, video, rect, pixels};

use game_field::field::{CELL_WIDTH, CELL_HEIGHT};
use constants::{BackgroundColor};
use constants::BackgroundColor::{White};

pub trait Draw_Myself {
    fn draw_myself(cell_status: CellStatus, renderer: &mut render::Canvas<video::Window>) -> () {
        let color = BackgroundColor::from_i16(cell_status.cell_type as i16).unwrap().value();
        let _ = renderer.set_draw_color(color as pixels::Color);

        let rect = rect::Rect::new(cell_status.x, cell_status.y, cell_status.width, cell_status.height);
        let _ = renderer.fill_rect(rect);
    }
}

pub struct Normal {}
impl Draw_Myself for Normal{}
pub struct Block {}
impl Draw_Myself for Block{}
pub struct Damage {}
impl Draw_Myself for Damage{}
pub struct Wall {}
impl Draw_Myself for Wall{}

pub struct Item {}
impl Draw_Myself for Item {
    fn draw_myself(cell_status: CellStatus, renderer: &mut render::Canvas<video::Window>) -> () {
        println!("{}", 5);
    }
}

pub struct CellFeature {}
impl CellFeature {
    fn new() -> CellFeature {
        return CellFeature{};
    }
    fn draw_myself(cell_status: CellStatus, renderer: &mut render::Canvas<video::Window>) -> () {
        match cell_status.cell_type {
            CellType::Normal => { Normal::draw_myself(cell_status, renderer); },
            CellType::Block => { Block::draw_myself(cell_status, renderer); },
            CellType::Damage => { Damage::draw_myself(cell_status, renderer); },
            CellType::Wall => { Wall::draw_myself(cell_status, renderer); },
            CellType::Item => { Item::draw_myself(cell_status, renderer); }
        }
    }
}

pub struct CellStatus {
    pub width: u32,
    pub height: u32,
    pub x: i32,
    pub y: i32,
    pub color: pixels::Color,
    pub cell_type: CellType
}
impl CellStatus {
    pub fn new(cell_type: i16, row_index: usize, cell_index: usize) -> CellStatus {
        return CellStatus {
            x: (cell_index * CELL_WIDTH as usize) as i32,
            y: (row_index * CELL_HEIGHT as usize) as i32,
            width: CELL_WIDTH as u32,
            height: CELL_HEIGHT as u32,
            color: White.value() as pixels::Color,
            cell_type: CellType::from_i16(cell_type).unwrap()
        };
    }
}

enum_from_primitive! {
    #[derive(Copy)]
    pub enum CellType {
        Normal, Block, Damage, Wall, Item
    }
}
impl Clone for CellType {
    fn clone(&self) -> CellType { *self }
}

pub struct FieldCell {
    pub status: CellStatus,
    pub feature: CellFeature
}

impl FieldCell {
    pub fn new(cell_type: i16, row_index: usize, cell_index: usize) -> FieldCell {
        FieldCell {
            status: CellStatus::new(cell_type, row_index, cell_index),
            feature: CellFeature::new(),
        }
    }

    pub fn draw(&self, renderer: &mut render::Canvas<video::Window>) -> () {}
}