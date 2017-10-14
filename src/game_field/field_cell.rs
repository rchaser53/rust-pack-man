use num::FromPrimitive;
use sdl2::{render, video, rect, pixels};

use game_field::field::{CELL_WIDTH, CELL_HEIGHT};
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

enum_from_primitive! {
    #[derive(Copy)]
    pub enum CellType {
        Normal, Block, Damage, Wall
    }
}
impl Clone for CellType {
    fn clone(&self) -> CellType { *self }
}
impl CellType {
    fn draw_myself(&self, renderer: &mut render::Canvas<video::Window>) -> () {
        match *self {
            CellType::Normal => {
                println!(1);
            },
            CellType::Block => {
                println!(2);
            },
            CellType::Damage => {
                println!(3);
            },
            CellType::Wall => {
                println!(4);
            }
        }
    }
}


// pub trait Draw_Myself {
//     fn draw_myself() -> ();
// }
// pub struct Normal {}
// impl Draw_Myself for Normal {
//     fn draw_myself() -> () {
//         println!("{}", 1);
//     }
// }

// pub struct Block {}
// impl Draw_Myself for Block {
//     fn draw_myself() -> () {
//         println!("{}", 2);
//     }
// }

// pub struct Damage {}
// impl Draw_Myself for Damage {
//     fn draw_myself() -> () {
//         println!("{}", 3);
//     }
// }

// pub struct Wall {}
// impl Draw_Myself for Wall {
//     fn draw_myself() -> () {
//         println!("{}", 4);
//     }
// }

// pub struct Goal {}
// impl Draw_Myself for Goal {
//     fn draw_myself() -> () {
//         println!("{}", 5);
//     }
// }

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
        // self.status.cell_type.draw_myself(renderer);
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