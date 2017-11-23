use sdl2::gfx::primitives::DrawRenderer;
use sdl2::{render, video};

use game_field::field::{Field, FieldMetadata};
use hitbox::Hitbox;
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

  fn should_increment(&self, circle_position: i16, enemy_position: i16) -> bool {
    0 < (circle_position - enemy_position)
  }

  fn should_go_y(&self, field: &Field, enemy_status: &mut EnemyStatus) -> bool {
    FieldGetter::should_go_y(field, &enemy_status.hitbox, &field.circle.status.hitbox)
  }

  fn get_directions_to_circle(&self, circle_hitbox: &Hitbox, enemy_hitbox: &Hitbox) -> (Direction, Direction) {
    let x_direction = if self.should_increment(circle_hitbox.x, enemy_hitbox.x)
                      { East } else { West };
    let y_direction = if self.should_increment(circle_hitbox.y, enemy_hitbox.y)
                      { South } else { North };
    (x_direction, y_direction)
  }

  fn change_direction(&self, field: &Field, enemy_status: &mut EnemyStatus) {
    let circle_hitbox = &field.circle.status.hitbox;
    let search_field = FieldGetter::get_search_field(field, enemy_status);
    let (x_direction, y_direction) = self.get_directions_to_circle(circle_hitbox, &enemy_status.hitbox);

    if self.should_go_y(field, enemy_status) {
      if self.should_increment(circle_hitbox.y, enemy_status.hitbox.y) {
        self.avoid_wall(search_field.search_row[3].cell_types[2], enemy_status, South, x_direction);
      } else {
        self.avoid_wall(search_field.search_row[1].cell_types[2], enemy_status, North, x_direction);
      }
    } else {
      if self.should_increment(circle_hitbox.x, enemy_status.hitbox.x) {
        self.avoid_wall(search_field.search_row[2].cell_types[3], enemy_status, East, y_direction);
      } else {
        self.avoid_wall(search_field.search_row[2].cell_types[1], enemy_status, West, y_direction);
      }
    }
  }

  fn avoid_wall(&self, cell_type: CellType, enemy_status: &mut EnemyStatus, direction: Direction, other_direction: Direction) {
    if cell_type == Block || cell_type == Wall {
      enemy_status.direction = other_direction;
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