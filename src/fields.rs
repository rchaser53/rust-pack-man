use num::FromPrimitive;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

use sdl2::{render, video, rect, pixels};

use mixer_music::play_sound_effect;
use error_handling::{Result as CustomResult, GameOverError};
use constants::{BackgroundColor};
use constants::BackgroundColor::{White};
use game_status::{GameStatus};
use circle::{Circle};

pub const SCREEN_WIDTH: i16 = 600;
pub const SCREEN_HEIGHT: i16 = 600;
const CELL_WIDTH: i16 = 30;
const CELL_HEIGHT: i16 = 30;
const COLUMUNS_NUMBER: i16 = SCREEN_WIDTH / CELL_WIDTH;
const ROWS_NUMBER: i16 = SCREEN_HEIGHT / CELL_HEIGHT;

const SQUARE_MAP_PATH: &'static str = "assets/maps/sample_map1.txt";

enum_from_primitive! {
    #[derive(Copy)]
    pub enum CellType {
        Normal, Block, Damage, Wall
    }
}
impl Clone for CellType {
    fn clone(&self) -> CellType { *self }
}

pub fn read_file(file_name: &str) -> String {
    let mut file = File::open(file_name).expect("file not found");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    return contents
}

pub struct Field {
    pub field_rows: Vec<FieldRow>,
    pub circle: Circle,
    pub game_status: GameStatus
}
impl Field  {
    pub fn new() -> Field {
        let mut rows: Vec<FieldRow> = Vec::new();

        let map_config = read_file(SQUARE_MAP_PATH);
        let row_defs: Vec<&str> = map_config.split("\n").collect();
        for row_def in row_defs {
            rows.push(FieldRow::new(&row_def));
        }

        return Field {
            field_rows: rows,
            circle: Circle::new(),
            game_status: GameStatus::new()
        };
    }

    pub fn renew(&mut self, renderer: &mut render::Canvas<video::Window>) -> CustomResult<()> {
        if self.game_status.is_pause { return Ok(()); }

        self.draw_row(renderer);

        {
            let current_cell = self.get_current_cell()?;
            self.take_action_from_cell(&current_cell)?;
        }

        return Ok(self.circle.renew(renderer));
    }

    pub fn draw_row(&self, renderer: &mut render::Canvas<video::Window>) -> () {
        let rows = self.field_rows.iter();

        for (row_index, row) in rows.enumerate() {
            let cells = row.field_cells.iter();
            for (cell_index, cell) in cells.enumerate() {

                let color = BackgroundColor::from_i16(cell.cell_type as i16).unwrap().value();
                let _ = renderer.set_draw_color(color as pixels::Color);

                let x = (cell_index * cell.width as usize) as i32;
                let y = (row_index * cell.height as usize) as i32;

                let rect = rect::Rect::new(x, y, cell.width, cell.height);
                let _ = renderer.fill_rect(rect);
            }
        }
    }

    pub fn get_current_cell(&self) -> Result<&FieldCell, GameOverError> {
        let column = (self.circle.x * COLUMUNS_NUMBER) / SCREEN_WIDTH;
        let row = (self.circle.y * ROWS_NUMBER) / SCREEN_HEIGHT;

        if self.is_outof_frame(row, column) {
            return Err(GameOverError::OtherError("out of the frame"));
        }

        return Ok(&self.field_rows[row as usize].field_cells[column as usize]);
    }

    pub fn is_outof_frame(&self, row: i16, column: i16) -> bool {
        return row < 0 || (self.field_rows.len() as i16 - 1) < row
                || column < 0 || (self.field_rows[0].field_cells.len() as i16 - 1) < column;
    }

    pub fn take_action_from_cell(&self, current_cell: &FieldCell) -> Result<(), GameOverError> {
        match current_cell.cell_type {
            CellType::Damage => Err(hit_enemy()),
            CellType::Wall => Err(hit_wall()),
            _ => Ok(())
        }
    }
}

const HIT_EFFECT_PATH: &'static str = "assets/musics/sine.wav";
const HIT_ENEMY_MESSAGE: &'static str = "hit the enemy";
const HIT_ENEMY_WALL: &'static str = "hit the wall";

pub fn hit_enemy() -> GameOverError {
    let _ = play_sound_effect(Path::new(&HIT_EFFECT_PATH));
    return GameOverError::OtherError(HIT_ENEMY_MESSAGE);
}

pub fn hit_wall() -> GameOverError {
    let _ = play_sound_effect(Path::new(&HIT_EFFECT_PATH));
    return GameOverError::OtherError(HIT_ENEMY_WALL);
}

pub struct FieldRow {
    pub field_cells: Vec<FieldCell>,
}
impl FieldRow {
    pub fn new(row_def: &str) -> FieldRow {
        let mut cells: Vec<FieldCell> = Vec::new();

        let cell_defs: Vec<char> = row_def.chars().collect();
        for cell_def in cell_defs {
            cells.push(FieldCell::new(
                FieldRow::get_cell_type_from_charactor(cell_def)
            ));
        }
        
        return FieldRow {
            field_cells: cells
        };
    }

    pub fn get_cell_type_from_charactor(character: char) -> i16 {
        return match character {
            ' ' => 0,
            '#' => 2,
            '?' => 3,
            _ => 1
        };
    }
}

pub struct FieldCell {
    pub width: u32,
    pub height: u32,
    pub color: pixels::Color,
    pub cell_type: CellType
}

impl FieldCell {
    pub fn new(cell_type: i16) -> FieldCell {
        FieldCell {
            width: CELL_WIDTH as u32,
            height: CELL_HEIGHT as u32,
            color: White.value() as pixels::Color,
            cell_type: CellType::from_i16(cell_type).unwrap()
        }
    }
}