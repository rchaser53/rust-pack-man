use std::process;
use std::fs::File;
use std::io::prelude::*;

use sdl2::{render, video};
use sdl2::messagebox::{show_simple_message_box, MESSAGEBOX_INFORMATION};

use constants::{FILE_PATHS, CellType};
use constants::Direction::{East, West, South, North};
use constants::CellType::{Block, Damage, Wall, Item};

use error_handling::{Result as CustomResult, GameOverError};
use collision_handler::{CollisionFrame};
use game_status::{GameStatus};
use circle::{Circle};
use game_field::field_row::FieldRow;
use enemy::enemy::{Enemy, EnemyCreater};

pub const SCREEN_WIDTH: i16 = 600;
pub const SCREEN_HEIGHT: i16 = 600;
pub const CELL_WIDTH: i16 = 30;
pub const CELL_HEIGHT: i16 = 30;
pub const COLUMUNS_NUMBER: i16 = SCREEN_WIDTH / CELL_WIDTH;
pub const ROWS_NUMBER: i16 = SCREEN_HEIGHT / CELL_HEIGHT;

const GAME_CLEAR: &'static str = "Game Clear!";

pub fn read_file(file_name: &str) -> String {
    let mut file = File::open(file_name).expect("file not found");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    contents
}

pub struct Field {
    pub field_rows: Vec<FieldRow>,
    pub enemies: Vec<Enemy>,
    pub circle: Circle,
    pub game_status: GameStatus
}

impl Field  {
    pub fn new() -> Field {
        let map_config = read_file(FILE_PATHS.get("SQUARE_MAP_PATH").unwrap());
        Field {
            field_rows: Field::create_field_rows(&map_config),
            enemies: Field::create_enemies(&map_config),
            circle: Circle::new(),
            game_status: GameStatus::new()
        }
    }

    pub fn create_field_rows(map_config: &str) -> Vec<FieldRow> {
        let mut rows: Vec<FieldRow> = Vec::new();
        let row_defs: Vec<&str> = map_config.split('\n').collect();
        let row_defs_length = row_defs.len();
        for row_index in 0..row_defs_length {
            rows.push(FieldRow::new(row_defs[row_index], row_index));
        }
        rows
    }

    pub fn create_enemies(map_config: &str) -> Vec<Enemy> {
        let mut enemies: Vec<Enemy> = Vec::new();
        let row_defs: Vec<&str> = map_config.split('\n').collect();
        let row_defs_length = row_defs.len();
        for row_index in 0..row_defs_length {
            enemies.append(&mut EnemyCreater::create_enemy(row_defs[row_index], row_index));
        }
        enemies
    }

    pub fn renew(&mut self, renderer: &mut render::Canvas<video::Window>) -> CustomResult<()> {
        if self.is_game_clear() {
            let _ = show_simple_message_box(MESSAGEBOX_INFORMATION, GAME_CLEAR, GAME_CLEAR, None);
            process::exit(0);
        }

        if self.game_status.is_pause { return Ok(()) }

        let current_cell = &self.get_current_cell_type()?;
        self.take_action_from_cell_type(current_cell)?;

        self.renew_each(renderer);
        Ok(())
    }

    pub fn is_game_clear(&self) -> bool {
        let rows = self.field_rows.iter();
        for row in rows {
            let cells = row.field_cells.iter();
            for cell in cells {
                if cell.status.exist_item { return false }
            }
        }
        true
    }

    pub fn take_action_from_cell_type(&mut self, current_cell_type: &CellType) -> Result<(), GameOverError> {
        match *current_cell_type {
            Damage | Block => Err(CollisionFrame::hit_enemy()),
            Wall => {
                self.circle.is_stoped = true;
                Ok(())
            },
            Item => {
                let (row, column) = self.get_next_cell_index();
                self.field_rows[row as usize].field_cells[column as usize].status.exist_item = false;
                Ok(())
            },
            _ => {
                self.circle.is_stoped = false;
                Ok(())
            }
        }
    }

    pub fn renew_each(&mut self, renderer: &mut render::Canvas<video::Window>) {
        self.renew_rows(renderer);
        self.renew_enemies(renderer);
        self.circle.renew(renderer);
    }

    pub fn renew_rows(&self, renderer: &mut render::Canvas<video::Window>) {
        let rows = self.field_rows.iter();
        for row in rows {
            let cells = row.field_cells.iter();
            for cell in cells {
                cell.draw(renderer);
            }
        }
    }

    pub fn renew_enemies(&mut self, renderer: &mut render::Canvas<video::Window>) {
        for enemy in &mut self.enemies {
            enemy.feature.renew(&mut enemy.status, renderer);
        }
    }

    pub fn get_current_cell_type(&self) -> Result<CellType, GameOverError> {
        let (row, column) = self.get_next_cell_index();

        if self.is_outof_frame(row, column) {
            return Err(GameOverError::OtherError("out of the frame"));
        }

        Ok(self.field_rows[row as usize].field_cells[column as usize].status.cell_type.clone())
    }

    pub fn is_outof_frame(&self, row: i16, column: i16) -> bool {
        row < 0 || (self.field_rows.len() as i16 - 1) < row
         || column < 0 || (self.field_rows[0].field_cells.len() as i16 - 1) < column
    }

    pub fn get_next_cell_index(&self) -> (i16, i16) {
        let column: f32 = (self.circle.x * COLUMUNS_NUMBER) as f32 / SCREEN_WIDTH as f32;
        let row: f32 = (self.circle.y * ROWS_NUMBER) as f32 / SCREEN_HEIGHT as f32;

        match self.circle.direction {
            num if num == East.value() => (row as i16, column.round() as i16),
            num if num == South.value() => (row.round() as i16, column as i16),
            num if num == West.value() => (row as i16, column.round() as i16 - 1),
            num if num == North.value() => (row.round() as i16 - 1, column as i16),
            _ => (column as i16, row as i16)
        }
    }
}