use num::FromPrimitive;
use sdl2::{render, video, rect, pixels};

use game_field::field_cell::{CellStatus, CellType};
use constants::{BackgroundColor};

pub trait DrawMyself {
    fn draw_myself(cell_status: &CellStatus, renderer: &mut render::Canvas<video::Window>) -> () {
        let color = BackgroundColor::from_i16(cell_status.cell_type as i16).unwrap().value();
        let _ = renderer.set_draw_color(color as pixels::Color);

        let rect = rect::Rect::new(cell_status.x, cell_status.y, cell_status.width, cell_status.height);
        let _ = renderer.fill_rect(rect);
    }
}

pub struct Normal {}
impl DrawMyself for Normal{}
pub struct Block {}
impl DrawMyself for Block{}
pub struct Damage {}
impl DrawMyself for Damage{}
pub struct Wall {}
impl DrawMyself for Wall{}

pub struct Item {}
impl DrawMyself for Item {
    fn draw_myself(cell_status: &CellStatus, renderer: &mut render::Canvas<video::Window>) -> () {
        println!("{}", 5);
    }
}

pub struct CellFeature {}
impl CellFeature {
    pub fn draw(cell_status: &CellStatus, renderer: &mut render::Canvas<video::Window>) -> () {
        match cell_status.cell_type {
            CellType::Normal => { Normal::draw_myself(cell_status, renderer); },
            CellType::Block => { Block::draw_myself(cell_status, renderer); },
            CellType::Damage => { Damage::draw_myself(cell_status, renderer); },
            CellType::Wall => { Wall::draw_myself(cell_status, renderer); },
            CellType::Item => { Item::draw_myself(cell_status, renderer); }
        }
    }
}