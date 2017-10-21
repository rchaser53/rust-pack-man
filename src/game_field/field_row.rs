use game_field::field_cell::{FieldCell};

pub struct FieldRow {
    pub field_cells: Vec<FieldCell>,
}
impl FieldRow {
    pub fn new(row_def: &str, row_index: usize) -> FieldRow {
        let mut cells: Vec<FieldCell> = Vec::new();

        let cell_defs: Vec<char> = row_def.chars().collect();
        let cell_defs_length = cell_defs.len();
        for cell_index in 0..cell_defs_length {
            cells.push(FieldCell::new(
                FieldRow::get_cell_type_from_charactor(cell_defs[cell_index]),
                row_index,
                cell_index
            ));
        }

        FieldRow {
            field_cells: cells
        }
    }

    pub fn get_cell_type_from_charactor(character: char) -> i16 {
        match character {
            ' ' => 0,
            '#' => 2,
            '?' => 3,
            '*' => 4,
            _ => 1
        }
    }
}