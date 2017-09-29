use sdl2::{render, video, rect, pixels};
use rand::{thread_rng, Rng};

use constants::{BackgroundColor, Direction};
use constants::BackgroundColor::{Black, Aqua, White};
use constants::Direction::{East, West, South, North};

pub struct Field {
    field_rows: Vec<FieldRow>,
}
impl Field {
    pub fn new() -> Field {
        let mut rows: Vec<FieldRow> = Vec::new();
        for n in 0 .. 10 {
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
            for cell in cells {

                let _ = renderer.set_draw_color(White.value() as pixels::Color);
                let rect = rect::Rect::new(10, 10, 10, 10);
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
        for n in 0 .. 10 {
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
    width: i16,
    height: i16,
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