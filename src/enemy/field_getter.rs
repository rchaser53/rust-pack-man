use game_field::field::Field;

use enemy::enemy_status::{EnemyStatus};
use hitbox::Hitbox;
use constants::{CellType};
use constants::CellType::{
  Block, Wall
};

pub struct SearchField {
  pub search_row: Vec<SearchRow>
}

#[derive(Debug)]
pub struct SearchRow {
  pub cell_types: Vec<CellType>
}

pub struct FieldGetter {}
impl FieldGetter {
  pub fn get_search_field(field: &Field, enemy_status: &EnemyStatus) -> SearchField {
    let (row, column) = field.position_handler.get_a_bit_next_position(&enemy_status.hitbox, &enemy_status.direction);
    let mut row_vec: Vec<SearchRow> = Vec::new();
    let seed_indexs: [i16; 5] = [-2, -1, 0, 1, 2];

    for seed_index in seed_indexs.into_iter() {
      if 0 < (row as i16 + seed_index)
          && (row as i16 + seed_index) < field.position_handler.row_number {
        let row_index = seed_index + row as i16;
        row_vec.push(FieldGetter::get_seach_row(&field, row_index as usize, column));
      } else {
        let mut cell_vec: Vec<CellType> = Vec::new();
        for _ in 0..field.position_handler.row_number {
          cell_vec.push(Block);
        }

        row_vec.push(SearchRow{
          cell_types: cell_vec
        });
      }
    }
    SearchField { search_row: row_vec }
  }

  pub fn get_seach_row(field: &Field, row: usize, column: usize) -> SearchRow {
    let mut cell_vec: Vec<CellType> = Vec::new();
    let seed_indexs: [i16; 5] = [-2, -1, 0, 1, 2];

    for seed_index in seed_indexs.into_iter() {
      if 0 < (column as i16 + seed_index)
          && (column as i16 + seed_index) < field.position_handler.column_number {
        let column_index = seed_index + column as i16;
        cell_vec.push(field.get_cell_type(row, column_index as usize));
      } else {
        cell_vec.push(Wall);
      }
    }
    SearchRow { cell_types: cell_vec }
  }

  pub fn get_distance_from_enemy_to_circle(field: &Field, enemy_hit_box: &Hitbox, circle_hit_box: &Hitbox) -> (i16, i16) {
    let (enemy_row, enemy_column) = field.position_handler.get_current_cell_position(enemy_hit_box.x, enemy_hit_box.y);
    let (circle_row, circle_column) = field.position_handler.get_current_cell_position(circle_hit_box.x, circle_hit_box.y);

    (enemy_row as i16 - circle_row as i16, enemy_column as i16 - circle_column as i16)
  }

  pub fn should_go_y(field: &Field, enemy_hit_box: &Hitbox, circle_hit_box: &Hitbox) -> bool {
    let (row, column) = FieldGetter::get_distance_from_enemy_to_circle(field, enemy_hit_box, circle_hit_box);

    let (enemy_row, enemy_column) = field.position_handler.get_current_cell_position(enemy_hit_box.x, enemy_hit_box.y);
    if row.abs() <= column.abs() {
      for index in 0..column {
        let cell_type = field.get_cell_type(enemy_row, (enemy_column as i16 - index) as usize);
        if cell_type == Block && cell_type == Wall { return true; }
      }
      return false
    }

    for index in 0..row {
      let cell_type = field.get_cell_type((enemy_row as i16 - index) as usize, enemy_column);
      if cell_type == Block && cell_type == Wall { return false; }
    }
    true
  }
}



#[cfg(test)]
mod tests {
  use game_field::field::Field;
  use enemy::field_getter::FieldGetter;
  use enemy::enemy_status::{
    EnemyStatus,
    EnemyStatusFactory,
    NormalEnemyStatusFactory
  };

  fn get_field() -> Field {
    Field::new()
  }

  fn get_enemy_status(row_index: usize, cell_index: usize) -> EnemyStatus {
    NormalEnemyStatusFactory{}.create_enemy_status(row_index, cell_index)
  }

  #[test]
  fn get_inside_row() {
    let field = get_field();
    FieldGetter::get_seach_row(&field, 5, 5);
  }

  #[test]
  fn get_outside_row_below() {
    let field = get_field();
    FieldGetter::get_seach_row(&field, 1, 5);
  }

  #[test]
  fn get_outside_row_over() {
    let field = get_field();
    FieldGetter::get_seach_row(&field, 9, 5);
  }

  #[test]
  fn get_inside_field() {
    let field = get_field();
    let enemy_status = get_enemy_status(5, 5);
    FieldGetter::get_search_field(&field, &enemy_status);
  }

  #[test]
  fn get_outside_field_below() {
    let field = get_field();
    let enemy_status = get_enemy_status(1, 5);
    FieldGetter::get_search_field(&field, &enemy_status);
  }

  #[test]
  fn get_outside_field_over() {
    let field = get_field();
    let enemy_status = get_enemy_status(9, 5);
    FieldGetter::get_search_field(&field, &enemy_status);
  }
}