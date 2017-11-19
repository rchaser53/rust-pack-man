use sdl2::gfx::primitives::DrawRenderer;
use sdl2::{render, video};

use game_field::field::{Field, FieldMetadata};
use circle::circle::Circle;
use constants::Direction::{East, West, South, North};
use constants::CellType::{
  Block, Wall
};
use enemy::enemy_status::{EnemyStatus};

pub trait EnemyAction {
  fn renew(&self, field: &Field, enemy_status: &mut EnemyStatus, renderer: &mut render::Canvas<video::Window>) {
    self.update(&field, enemy_status);
    self.draw(enemy_status, renderer);
  }

  #[allow(unused)]
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

  fn should_stop_moving(&self, field: &Field, row: usize, column: usize) -> bool {
    let cell_type = field.field_rows[row].field_cells[column].status.borrow().cell_type;
    match cell_type {
      Block | Wall => true,
      _ => false
    }
  }

  fn move_enemy(&self, enemy_status: &mut EnemyStatus) {
    match enemy_status.direction {
      East => { enemy_status.hitbox.x += enemy_status.move_speed; },
      West => { enemy_status.hitbox.x -= enemy_status.move_speed; },
      North => { enemy_status.hitbox.y -= enemy_status.move_speed; },
      South => { enemy_status.hitbox.y += enemy_status.move_speed; }
    }
    enemy_status.move_count += enemy_status.move_speed;
  }

  fn initialize_movecount(&self, metadata: &FieldMetadata, enemy_status: &mut EnemyStatus) {
    match enemy_status.direction {
      East | West => {
        if metadata.cell_width <= enemy_status.move_count {
          enemy_status.move_count = 0;
        }
      },
      North | South => {
        if metadata.cell_height <= enemy_status.move_count {
          enemy_status.move_count = 0;
        }
      }
    }
  }
}

pub struct NormalFeature {}
impl EnemyAction for NormalFeature {
  fn update(&self, field: &Field, enemy_status: &mut EnemyStatus) {
    self.initialize_movecount(&field.metadata, enemy_status);

    if enemy_status.move_count == 0 {
      self.change_direction(&field.circle, enemy_status);
    }

    let (row, column) = self.get_next_index(field, enemy_status);
    if self.should_stop_moving(field, row, column) {
      return;
    }

    self.move_enemy(enemy_status);
  }
}

  // println!("row: {} - column: {} - cell: {:?} ", row, column, field.field_rows[row as usize].field_cells[column as usize].status.borrow().cell_type);