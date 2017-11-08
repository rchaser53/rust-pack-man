use std::cell::RefCell;
use sdl2::{render, video};

use game_field::cell_feature::{DrawMyself};
use game_field::cell_status::{CellStatus};

pub struct FieldCell {
    pub status: RefCell<CellStatus>,
    pub feature: Box<DrawMyself> 
}

impl FieldCell {
    pub fn new(status: CellStatus, feature: Box<DrawMyself>) -> FieldCell {
        FieldCell {
            status: RefCell::new(status),
            feature: feature
        }
    }

    pub fn draw(&self, renderer: &mut render::Canvas<video::Window>) {
        self.feature.draw(&self.status.borrow(), renderer);
    }
}