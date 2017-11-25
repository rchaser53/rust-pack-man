use std::process;

use sdl2::messagebox::{show_simple_message_box, MESSAGEBOX_INFORMATION};
use circle::circle::{Circle};
use enemy::enemy::{Enemy};
use error_handling::{GameOverError};
use constants::{HIT_ENEMY_MESSAGE, GAME_CLEAR};
use game_field::field_row::FieldRow;
use game_field::field::Field;

pub struct GameEventHandler {}
impl GameEventHandler {
    pub fn handle_game_event(field: &Field) -> Result<(), GameOverError> {
        if GameEventHandler::is_game_clear(&field.field_rows) {
            let _ = show_simple_message_box(MESSAGEBOX_INFORMATION, GAME_CLEAR, GAME_CLEAR, None);
            process::exit(0);
        }

        if GameEventHandler::is_hit_enemy(&field.circle, &field.enemies) {
            return Err(GameOverError::HitEnemy(HIT_ENEMY_MESSAGE));
        }

        Ok(())
    }

    pub fn is_game_clear(field_rows: &Vec<FieldRow>) -> bool {
        let rows = field_rows.iter();
        for row in rows {
            let cells = row.field_cells.iter();
            for cell in cells {
                if cell.status.borrow().exist_item { return false }
            }
        }
        true
    }

    pub fn is_hit_enemy(circle: &Circle, enemies: &Vec<Enemy>) -> bool {
        let enemies = enemies.iter();
        for enemy in enemies {
            let is_x_hit_range = (enemy.status.borrow().hitbox.x - circle.status.hitbox.x).abs() <= 20;
            let is_y_hit_range = (enemy.status.borrow().hitbox.y - circle.status.hitbox.y).abs() <= 20;

            if is_x_hit_range && is_y_hit_range {
                return true
            }
        }
        false
    }
}