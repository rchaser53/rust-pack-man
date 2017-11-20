use constants::{Direction, CELL_WIDTH, CELL_HEIGHT};
use constants::Direction::{East, West, South, North};
use error_handling::{GameOverError};
use hitbox::Hitbox;

pub const SCREEN_WIDTH: i16 = 600;
pub const SCREEN_HEIGHT: i16 = 600;
pub const COLUMUNS_NUMBER: i16 = SCREEN_WIDTH / CELL_WIDTH;
pub const ROWS_NUMBER: i16 = SCREEN_HEIGHT / CELL_HEIGHT;

pub struct PositionHandler {
    pub row_number: i16,
    pub column_number: i16
}

impl PositionHandler {
    pub fn new(row_number: i16, column_number: i16) -> PositionHandler {
        PositionHandler {
            row_number: row_number,
            column_number: column_number
        }
    }

    pub fn get_a_bit_next_position(&self, hitbox: &Hitbox, direction: &Direction) -> (usize, usize) {
        match *direction {
            East => self.get_current_cell_position(hitbox.x + hitbox.width, hitbox.y),
            West => self.get_current_cell_position(hitbox.x, hitbox.y),
            South => self.get_current_cell_position(hitbox.x, hitbox.y + hitbox.height),
            North => self.get_current_cell_position(hitbox.x, hitbox.y)
        }
    }

    pub fn get_current_cell_position(&self, x: i16, y: i16) -> (usize, usize) {
        let column = (x * COLUMUNS_NUMBER) as f32 / SCREEN_WIDTH as f32;
        let row = (y * ROWS_NUMBER) as f32 / SCREEN_HEIGHT as f32;
        (row as usize, column as usize)
    }

    pub fn get_next_cell_position(&self, hitbox: &Hitbox, direction: &Direction) -> Result<(usize, usize), GameOverError> {
        let (row, column) = self.get_next_cell_index_from_direction(hitbox, direction);

        if self.is_outof_frame(row, column) {
            return Err(GameOverError::OtherError("out of the frame"));
        }
        Ok((row as usize, column as usize))
    }

    pub fn is_outof_frame(&self, row: i16, column: i16) -> bool {
        row < 0 || (self.row_number - 1) < row
         || column < 0 || (self.column_number - 1) < column
    }

    pub fn get_next_cell_index_from_direction(&self, hitbox: &Hitbox, direction: &Direction) -> (i16, i16) {
        let column = (hitbox.x * COLUMUNS_NUMBER) as f32 / SCREEN_WIDTH as f32;
        let row = (hitbox.y * ROWS_NUMBER) as f32 / SCREEN_HEIGHT as f32;

        match *direction {
            East => {
                let column = (hitbox.x * COLUMUNS_NUMBER + hitbox.width) as f32 / SCREEN_WIDTH as f32;
                (row as i16, column.round() as i16)
            },
            West => (row as i16, column.round() as i16 - 1),
            South => {
                let row = (hitbox.y * ROWS_NUMBER + hitbox.height ) as f32 / SCREEN_HEIGHT as f32;
                (row.round() as i16, column as i16)
            },
            North => (row.round() as i16 - 1, column as i16)
        }
    }
}