use sdl2::gfx::primitives::DrawRenderer;
use sdl2::{render, video};
use rand;
use rand::Rng;

use game_field::field::Field;
use circle::circle::Circle;
use constants::{Direction, CellType};
use constants::Direction::{East, West, South, North};
use constants::CellType::{
  Normal, Block, Damage, Wall, Item
};
use enemy::enemy_status::{EnemyStatus};

pub struct CellInformation {
  cell_type: CellType,
  direction: Direction
}

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

  fn get_random_sign(&self) -> i16 {
    if rand::thread_rng().gen::<bool>() {
      return 1
    }
    -1
  }

  fn is_correct_index(&self, row: i16, column: i16, field: &Field) -> bool {
    if row < 0 || column < 0 {
      return false;
    }

    if field.field_rows.len() <= row as usize || field.field_rows[0].field_cells.len() <= column as usize {
      return false;
    }

    true
  }

  fn change_index_by_direction(&self, row: i16, column: i16, direction: &Direction) -> (i16, i16) {
    match *direction {
      East => (row, column + 1),
      South => (row + 1, column),
      West => (row - 1, column),
      North => (row, column - 1)
    }
  }

  fn convert_next_direction(&self, direction: Direction) -> Direction {
    match direction {
      East => South,
      South => West,
      West => North,
      North => East
    }
  }

  fn get_next_index(&self, field: &Field, enemy_status: &mut EnemyStatus) -> (usize, usize) {
    let mut continuous_flag = true;
    let mut return_row = 0;
    let mut return_column = 0;
    let circle = &field.circle;

    self.change_direction(circle, enemy_status);
    while continuous_flag {
      let (row, column) = field.position_handler
                              .get_target_cell_index(enemy_status.hitbox.x, enemy_status.hitbox.y);
      let (next_row, next_column) = self.change_index_by_direction(row, column, &enemy_status.direction);
      return_row = next_row;
      return_column = next_column;
      continuous_flag = !self.is_correct_index(return_row, return_column, &field);
    }

    (return_row as usize, return_column as usize)
  }

}

pub struct NormalFeature {}
impl EnemyAction for NormalFeature {
  fn update(&self, field: &Field, enemy_status: &mut EnemyStatus) {
    let circle = &field.circle;
    let (row, column) = self.get_next_index(field, enemy_status);

    match field.field_rows[row].field_cells[column].status.borrow().cell_type {
      Block => { return; }
      _ => {}
    };

    match enemy_status.direction {
      East | West => {
        enemy_status.hitbox.x += self.calculate_x_moving_distance(circle, enemy_status);
      },
      _ => {
        enemy_status.hitbox.y += self.calculate_y_moving_distance(circle, enemy_status);
      }
    }
  }
}
    // println!("row: {} - column: {} - cell: {:?} ", row, column, field.field_rows[row].field_cells[column].status.borrow().cell_type);