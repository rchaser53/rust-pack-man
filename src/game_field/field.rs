use std::fs::File;
use std::io::prelude::*;

use sdl2::{render, video};

use constants::{FILE_PATHS, CELL_WIDTH, CELL_HEIGHT, CellType};

use error_handling::{Result as CustomResult};
use game_status::{GameStatus};
use circle::circle::{Circle};
use game_field::field_row::FieldRow;
use game_field::position_handler::PositionHandler;
use game_field::game_event_handler::GameEventHandler;
use enemy::enemy::{Enemy, EnemyCreater};

pub fn read_file(file_name: &str) -> String {
    let mut file = File::open(file_name).expect("file not found");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    contents
}

pub struct FieldMetadata {
    pub cell_width: i16,
    pub cell_height: i16
}

pub struct Field {
    pub field_rows: Vec<FieldRow>,
    pub enemies: Vec<Enemy>,
    pub circle: Circle,
    pub game_status: GameStatus,
    pub position_handler: PositionHandler,
    pub metadata: FieldMetadata
}

impl Field  {
    pub fn new() -> Field {
        let map_config = read_file(FILE_PATHS.get("SQUARE_MAP_PATH").unwrap());
        let field_rows = Field::create_field_rows(&map_config);
        let row_number = field_rows.len() as i16;
        let column_number = field_rows[0].field_cells.len() as i16;

        Field {
            field_rows: field_rows,
            enemies: Field::create_enemies(&map_config),
            circle: Circle::new(),
            game_status: GameStatus::new(),
            position_handler: PositionHandler::new(row_number, column_number),
            metadata: FieldMetadata {
                cell_width: CELL_WIDTH, cell_height: CELL_HEIGHT
            }
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
        if self.game_status.is_pause { return Ok(()) }
        let _ = GameEventHandler::handle_game_event(&self).map_err(|err| println!("{}", err));
        let (row, column) = self.position_handler
                                .get_next_cell_position(&self.circle.status.hitbox, &self.circle.status.direction)?;
        self.take_action_by_cell(row, column)?;

        self.renew_each(renderer);
        Ok(())
    }

    pub fn take_action_by_cell(&mut self, row: usize, column: usize) -> CustomResult<()> {
        let target_cell = &mut self.field_rows[row as usize].field_cells[column as usize];
        let target_cell_feature = &target_cell.feature;
        let mut target_cell_status = &mut target_cell.status.borrow_mut();

        target_cell_feature.effect(&mut self.circle, &mut target_cell_status)?;
        Ok(())
    }

    pub fn renew_each(&mut self, renderer: &mut render::Canvas<video::Window>) {
        self.renew_rows(renderer);
        self.renew_enemies(renderer);
        self.circle.feature.renew(&mut self.circle.status, renderer);
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

    pub fn renew_enemies(&self, renderer: &mut render::Canvas<video::Window>) {
        for enemy in &self.enemies {
            enemy.feature.renew(self, &mut enemy.status.borrow_mut(), renderer);
        }
    }

    pub fn get_cell_type(&self, row: usize, column: usize) -> CellType {
        self.field_rows[row].field_cells[column].status.borrow().cell_type
    }
}
