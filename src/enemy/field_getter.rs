use game_field::field::Field;

use enemy::enemy_status::{EnemyStatus};
use constants::CellType;
use constants::CellType::{
  Normal, Block, Damage, Wall, Item
};

pub struct SeachField {
  search_row: Vec<SearchRow>
}

#[derive(Debug)]
pub struct SearchRow {
  cell_type: Vec<CellType>
}

pub struct FieldGetter {}
impl FieldGetter {
  // pub fn Hoge(field: Field, enemy_status: EnemyStatus) -> (usize, usize) {
  //   let (row, column) = field.position_handler.get_current_cell_position(&enemy_status.hitbox);

  // }

  pub fn get_seach_row(field: Field, row: usize, column: usize) -> Vec<CellType> {
    let mut cell_vec: Vec<CellType> = Vec::new();
    let seed_indexs: [i16; 5] = [-2, -1, 0, 1, 2];

    for seed_index in seed_indexs.into_iter() {
      if 0 < (column as i16 + seed_index)
          && (column as i16 + seed_index) < field.position_handler.column_number {
        cell_vec.push(field.field_rows[row].field_cells[column].status.borrow().cell_type);
      } else {
        cell_vec.push(Wall);
      }
    }
    cell_vec
  }
}



#[cfg(test)]
mod tests {
  use game_field::field::Field;
  use enemy::field_getter::FieldGetter;

  fn get_field() -> Field {
    Field::new()
  }

  #[test]
  fn get_inside_row() {
    let field = get_field();
    FieldGetter::get_seach_row(field, 5, 5);
  }

  #[test]
  fn get_outside_row_below() {
    let field = get_field();
    FieldGetter::get_seach_row(field, 1, 5);
  }

  #[test]
  fn get_outside_row_over() {
    let field = get_field();
    FieldGetter::get_seach_row(field, 9, 5);
  }
}


