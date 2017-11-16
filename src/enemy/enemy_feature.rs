use sdl2::gfx::primitives::DrawRenderer;
use sdl2::{render, video};

use game_field::field::Field;
use circle::circle::Circle;
use constants::{Direction, CellType};
use constants::Direction::{East, West, South, North};
use constants::CellType::{
  Normal, Block, Damage, Wall, Item
};
use enemy::enemy_status::{EnemyStatus};

pub const REACTION_RATE: i16 = 10;

pub trait EnemyAction {
  fn renew(&self, field: &Field, enemy_status: &mut EnemyStatus, renderer: &mut render::Canvas<video::Window>) {
    self.update(&field, enemy_status);
    self.draw(enemy_status, renderer);
  }

  fn update(&self, field: &Field, enemy_status: &mut EnemyStatus) {}

  fn draw(&self, enemy_status: &EnemyStatus, renderer: &mut render::Canvas<video::Window>) {
    let _ = renderer.filled_ellipse(enemy_status.hitbox.x, enemy_status.hitbox.y,
                                    enemy_status.hitbox.width, enemy_status.hitbox.height, enemy_status.background_color);
  }

  fn get_direction(&self, circle_position: i16, enemy_position: i16) -> i16 {
    if 0 < (circle_position - enemy_position) {
      return 1
    }
    -1
  }

  fn calculate_x_moving_distance(&self, circle: &Circle, enemy_status: &mut EnemyStatus) -> i16 {
    self.get_direction(circle.status.hitbox.x, enemy_status.hitbox.x) * enemy_status.move_speed
  }

  fn calculate_y_moving_distance(&self, circle: &Circle, enemy_status: &mut EnemyStatus) -> i16 {
    self.get_direction(circle.status.hitbox.y, enemy_status.hitbox.y) * enemy_status.move_speed
  }

  fn change_direction(&self, circle: &Circle, enemy_status: &mut EnemyStatus) {
    if (circle.status.hitbox.x - enemy_status.hitbox.x).abs()
          < (circle.status.hitbox.y - enemy_status.hitbox.y).abs() {
      if self.get_direction(circle.status.hitbox.y, enemy_status.hitbox.y) == 1 {
        enemy_status.direction = South;
      } else {
        enemy_status.direction = North;
      }
    } else {
      if self.get_direction(circle.status.hitbox.x, enemy_status.hitbox.x) == 1 {
        enemy_status.direction = East;
      } else {
        enemy_status.direction = West;
      }
    }
  }

  fn get_next_index(&self, field: &Field, enemy_status: &mut EnemyStatus) -> (usize, usize) {
    let (row, column) = field.position_handler.get_next_cell_index_from_direction(&enemy_status.hitbox, &enemy_status.direction);

    (row as usize, column as usize)
  }

  fn should_change_direction(&self, field: &Field, row: usize, column: usize) -> bool {
    match self.get_cell_type(field, row, column) {
      Block => true,
      _ => false
    }
  }

  fn get_cell_type(&self, field: &Field, row: usize, column: usize) -> CellType {
    field.field_rows[row].field_cells[column].status.borrow().cell_type
  }
}

pub struct NormalFeature {}
impl EnemyAction for NormalFeature {
  fn update(&self, field: &Field, enemy_status: &mut EnemyStatus) {
    let circle = &field.circle;
    let (row, column) = self.get_next_index(field, enemy_status);

    if enemy_status.move_count < REACTION_RATE {
      enemy_status.move_count += 1;
    } else {
      self.change_direction(circle, enemy_status);
      enemy_status.move_count = 1;
    }

    if self.should_change_direction(field, row, column) {
      return;
    }

    match enemy_status.direction {
      East => { enemy_status.hitbox.x += enemy_status.move_speed; },
      West => { enemy_status.hitbox.x -= enemy_status.move_speed; },
      North => { enemy_status.hitbox.y -= enemy_status.move_speed; },
      South => { enemy_status.hitbox.y += enemy_status.move_speed; }
    }
  }
}

  // println!("row: {} - column: {} - cell: {:?} ", row, column, field.field_rows[row as usize].field_cells[column as usize].status.borrow().cell_type);