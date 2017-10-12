use num::FromPrimitive;
use sdl2::{render, video, rect, pixels};

use game_field::field::{CELL_WIDTH, CELL_HEIGHT, CellType};
use constants::{BackgroundColor};
use constants::BackgroundColor::{White};

pub struct CellStatus {
    pub cell_type: CellType
}
impl CellStatus {
    pub fn new(cell_type: i16) -> CellStatus {
        return CellStatus {
            cell_type: CellType::from_i16(cell_type).unwrap()
        };
    }
}

pub struct FieldCell {
    pub width: u32,
    pub height: u32,
    pub color: pixels::Color,
    pub status: CellStatus
}

impl FieldCell {
    pub fn new(cell_type: i16) -> FieldCell {
        FieldCell {
            width: CELL_WIDTH as u32,
            height: CELL_HEIGHT as u32,
            color: White.value() as pixels::Color,
            status: CellStatus::new(cell_type),
        }
    }

    pub fn draw(&self, renderer: &mut render::Canvas<video::Window>,
                row_index: usize, cell_index: usize) -> () {
        self.draw_background(renderer, row_index, cell_index);
    }

    pub fn draw_background( &self, renderer: &mut render::Canvas<video::Window>,
                            row_index: usize, cell_index: usize) -> () {
        let color = BackgroundColor::from_i16(self.status.cell_type as i16).unwrap().value();
        let _ = renderer.set_draw_color(color as pixels::Color);

        let x = (cell_index * self.width as usize) as i32;
        let y = (row_index * self.height as usize) as i32;

        let rect = rect::Rect::new(x, y, self.width, self.height);
        let _ = renderer.fill_rect(rect);
    }
}