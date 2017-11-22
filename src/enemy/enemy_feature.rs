use sdl2::gfx::primitives::DrawRenderer;
use sdl2::{render, video};

use game_field::field::{Field, FieldMetadata};
use circle::circle::Circle;
use constants::{CellType, Direction};
use constants::Direction::{East, West, South, North};
use constants::CellType::{
  Block, Wall
};
use enemy::enemy_status::{EnemyStatus};
use enemy::field_getter::{FieldGetter};

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

  fn should_go_y(&self, field: &Field, enemy_status: &mut EnemyStatus) -> bool {
    FieldGetter::should_go_y(field, &enemy_status.hitbox, &field.circle.status.hitbox)
  }

  fn change_direction(&self, field: &Field, enemy_status: &mut EnemyStatus) {
    let hitbox = &field.circle.status.hitbox;
    let search_field = FieldGetter::get_search_field(field, enemy_status);
    if self.should_go_y(field, enemy_status) {
      if self.get_direction(hitbox.y, enemy_status.hitbox.y) == 1 {
        self.avoid_wall(search_field.search_row[3].cell_types[2], South, enemy_status);
      } else {
        self.avoid_wall(search_field.search_row[1].cell_types[2], North, enemy_status);
      }
    } else {
      if self.get_direction(hitbox.x, enemy_status.hitbox.x) == 1 {
        self.avoid_wall(search_field.search_row[2].cell_types[3], East, enemy_status);
      } else {
        self.avoid_wall(search_field.search_row[2].cell_types[1], West, enemy_status);
      }
    }
  }

  fn avoid_wall(&self, cell_type: CellType, direction: Direction, enemy_status: &mut EnemyStatus) {
    if cell_type == Block || cell_type == Wall {
      enemy_status.direction = direction.clockwise();
    } else {
      enemy_status.direction = direction;
    }
  }

  fn get_next_index(&self, field: &Field, enemy_status: &mut EnemyStatus) -> (usize, usize) {
    let (row, column) = field.position_handler.get_next_cell_index_from_direction(&enemy_status.hitbox, &enemy_status.direction);
    (row as usize, column as usize)
  }

  fn should_stop_moving(&self, field: &Field, row: usize, column: usize) -> bool {
    match field.get_cell_type(row, column) {
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
    self.change_direction(&field, enemy_status);

    let (row, column) = self.get_next_index(field, enemy_status);
    if self.should_stop_moving(field, row, column) {
      return;
    }

    self.move_enemy(enemy_status);
  }
}

  // println!("row: {} - column: {} - cell: {:?} ", row, column, field.field_rows[row as usize].field_cells[column as usize].status.borrow().cell_type);