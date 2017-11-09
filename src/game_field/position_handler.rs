use constants::Direction::{East, West, South, North};
use error_handling::{GameOverError};

pub const SCREEN_WIDTH: i16 = 600;
pub const SCREEN_HEIGHT: i16 = 600;
pub const CELL_WIDTH: i16 = 30;
pub const CELL_HEIGHT: i16 = 30;
pub const COLUMUNS_NUMBER: i16 = SCREEN_WIDTH / CELL_WIDTH;
pub const ROWS_NUMBER: i16 = SCREEN_HEIGHT / CELL_HEIGHT;

pub struct PositionHandler {
    row_number: i16,
    column_number: i16
}

impl PositionHandler {
    pub fn new(row_number: i16, column_number: i16) -> PositionHandler {
        PositionHandler {
            row_number: row_number,
            column_number: column_number
        }
    }

    pub fn get_current_cell_position(&self, x: i16, y: i16, direction: i16) -> Result<(usize, usize), GameOverError> {
        let (row, column) = self.get_next_cell_index(x, y, direction);

        if self.is_outof_frame(row, column) {
            return Err(GameOverError::OtherError("out of the frame"));
        }
        Ok((row as usize, column as usize))
    }

    pub fn is_outof_frame(&self, row: i16, column: i16) -> bool {
        row < 0 || (self.row_number - 1) < row
         || column < 0 || (self.column_number - 1) < column
    }

    pub fn get_next_cell_index(&self, x: i16, y: i16, direction: i16) -> (i16, i16) {
        let column: f32 = (x * COLUMUNS_NUMBER) as f32 / SCREEN_WIDTH as f32;
        let row: f32 = (y * ROWS_NUMBER) as f32 / SCREEN_HEIGHT as f32;

        match direction {
            num if num == East.value() => (row as i16, column.round() as i16),
            num if num == South.value() => (row.round() as i16, column as i16),
            num if num == West.value() => (row as i16, column.round() as i16 - 1),
            num if num == North.value() => (row.round() as i16 - 1, column as i16),
            _ => (column as i16, row as i16)
        }
    }    
}