use game_field::field::Field;

use enemy::enemy_status::{EnemyStatus};
use constants::CellType;
use constants::CellType::{
  Normal, Block, Damage, Wall, Item
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
    FieldGetter::get_search_field(&field, enemy_status);
  }

  #[test]
  fn get_outside_field_below() {
    let field = get_field();
    let enemy_status = get_enemy_status(1, 5);
    FieldGetter::get_search_field(&field, enemy_status);
  }

  #[test]
  fn get_outside_field_over() {
    let field = get_field();
    let enemy_status = get_enemy_status(9, 5);
    FieldGetter::get_search_field(&field, enemy_status);
  }
}