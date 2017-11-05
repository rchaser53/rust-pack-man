use std::process;

use sdl2::messagebox::{show_simple_message_box, MESSAGEBOX_INFORMATION};
use error_handling::{Result as CustomResult};
use circle::{Circle};
use enemy::enemy::{Enemy};
use game_field::field_row::FieldRow;
use game_field::field::Field;

const GAME_CLEAR: &'static str = "Game Clear!";
const HIT_ENEMY: &'static str = "Hit Enemy!";

pub struct GameEventHandler {}
impl GameEventHandler {
    pub fn handle_game_event(field: &Field) -> CustomResult<()> {
        if GameEventHandler::is_game_clear(&field.field_rows) {
            let _ = show_simple_message_box(MESSAGEBOX_INFORMATION, GAME_CLEAR, GAME_CLEAR, None);
            process::exit(0);
        }

        if GameEventHandler::is_hit_enemy(&field.circle, &field.enemies) {
            let _ = show_simple_message_box(MESSAGEBOX_INFORMATION, HIT_ENEMY, HIT_ENEMY, None);
            process::exit(0);
        }

        Ok(())
    }

    pub fn is_game_clear(field_rows: &Vec<FieldRow>) -> bool {
        let rows = field_rows.iter();
        for row in rows {
            let cells = row.field_cells.iter();
            for cell in cells {
                if cell.status.exist_item { return false }
            }
        }
        true
    }

    pub fn is_hit_enemy(circle: &Circle, enemies: &Vec<Enemy>) -> bool {
        let enemies = enemies.iter();
        for enemy in enemies {
            let is_x_hit_range = (enemy.status.x - circle.x).abs() <= 20;
            let is_y_hit_range = (enemy.status.y - circle.y).abs() <= 20;

            if is_x_hit_range && is_y_hit_range {
                return true
            }
        }
        false
    }
}