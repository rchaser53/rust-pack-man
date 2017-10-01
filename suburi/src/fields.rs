use sdl2::{render, video, rect, pixels};
use rand::{thread_rng, Rng};

use constants::{BackgroundColor, Direction};
use constants::BackgroundColor::{Black, Aqua, White};
use constants::Direction::{East, West, South, North};

pub const SCREEN_WIDTH: i16 = 600;
pub const SCREEN_HEIGHT: i16 = 600;
const CELL_WIDTH: i16 = 30;
const CELL_HEIGHT: i16 = 30;
const COLUMUNS_NUMBER: i16 = SCREEN_WIDTH / CELL_WIDTH;
const ROWS_NUMBER: i16 = SCREEN_HEIGHT / CELL_HEIGHT;

pub struct Field {
    field_rows: Vec<FieldRow>,
}
impl Field {
    pub fn new() -> Field {
        let mut rows: Vec<FieldRow> = Vec::new();
        for n in 0 .. COLUMUNS_NUMBER {
            rows.push(FieldRow::new());
        }
        
        return Field {
            field_rows: rows
        };
    }

    pub fn draw(&self, renderer: &mut render::Canvas<video::Window>) -> () {
        let mut rows = self.field_rows.iter();

        for (rowIndex, row) in rows.enumerate() {
            let mut cells = row.field_cells.iter();
            for (cellIndex, cell) in cells.enumerate() {
                let _ = renderer.set_draw_color(White.value() as pixels::Color);

                let x = (cellIndex * cell.width as usize) as i32;
                let y = (rowIndex * cell.height as usize) as i32;

                let rect = rect::Rect::new(x, y, cell.width, cell.height);
                let _ = renderer.fill_rect(rect);
            }
        }
    }
}

pub struct FieldRow {
    field_cells: Vec<FieldCell>,
}
impl FieldRow {
    pub fn new() -> FieldRow {
        let mut cells: Vec<FieldCell> = Vec::new();
        for n in 0 .. ROWS_NUMBER {
            cells.push(FieldCell::new(
                thread_rng().gen_range(1, 3)
            ));
        }
        
        return FieldRow {
            field_cells: cells
        };
    }
}

pub struct FieldCell {
    width: u32,
    height: u32,
    color: pixels::Color
}

impl FieldCell {
    pub fn new(n: i16) -> FieldCell {
        FieldCell {
            width: 30,
            height: 30,
            color: White.value() as pixels::Color
        }
    }
}