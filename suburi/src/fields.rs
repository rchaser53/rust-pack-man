use num::FromPrimitive;
use num::rational::{Ratio};

use sdl2::{render, video, rect, pixels};
use rand::{thread_rng, Rng};

use constants::{BackgroundColor, Direction};
use constants::BackgroundColor::{Black, Aqua, White};
use circle::{Circle};

pub const SCREEN_WIDTH: i16 = 600;
pub const SCREEN_HEIGHT: i16 = 600;
const CELL_WIDTH: i16 = 30;
const CELL_HEIGHT: i16 = 30;
const COLUMUNS_NUMBER: i16 = SCREEN_WIDTH / CELL_WIDTH;
const ROWS_NUMBER: i16 = SCREEN_HEIGHT / CELL_HEIGHT;

enum_from_primitive! {
    #[derive(Copy, Debug)]
    pub enum CellType {
        Normal,
        Block,
        Damage
    }
}
impl Clone for CellType {
    fn clone(&self) -> CellType { *self }
}

pub struct Field<'a> {
    pub field_rows: Vec<FieldRow>,
    pub circle: &'a mut Circle
}
impl <'a>Field <'a> {
    pub fn new(circle: &'a mut Circle) -> Field {
        let mut rows: Vec<FieldRow> = Vec::new();
        for _ in 0 .. COLUMUNS_NUMBER {
            rows.push(FieldRow::new());
        }

        println!("{}", Ratio::from_integer(CellType::Block as i16));
        println!("{:?}", CellType::from_i16(1).unwrap());

        return Field {
            field_rows: rows,
            circle: circle
        };
    }

    pub fn draw(&self, renderer: &mut render::Canvas<video::Window>) -> () {
        self.draw_row(renderer);
    }

    pub fn draw_row(&self, renderer: &mut render::Canvas<video::Window>) -> () {
        let rows = self.field_rows.iter();

        for (row_index, row) in rows.enumerate() {
            let cells = row.field_cells.iter();
            for (cell_index, cell) in cells.enumerate() {
                let _ = renderer.set_draw_color(White.value() as pixels::Color);

                let x = (cell_index * cell.width as usize) as i32;
                let y = (row_index * cell.height as usize) as i32;

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
        for _ in 0 .. ROWS_NUMBER {
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
    color: pixels::Color,
    // cell_type: CellType
}

impl FieldCell {
    pub fn new(cell_type: i16) -> FieldCell {
        FieldCell {
            width: CELL_WIDTH as u32,
            height: CELL_HEIGHT as u32,
            color: White.value() as pixels::Color,
            // cell_type: num::FromPrimitive::from_i16(cell_type)
        }
    }
}