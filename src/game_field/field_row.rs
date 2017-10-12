use game_field::field_cell::{FieldCell};

pub struct FieldRow {
    pub field_cells: Vec<FieldCell>,
}
impl FieldRow {
    pub fn new(row_def: &str) -> FieldRow {
        let mut cells: Vec<FieldCell> = Vec::new();

        let cell_defs: Vec<char> = row_def.chars().collect();
        for cell_def in cell_defs {
            cells.push(FieldCell::new(
                FieldRow::get_cell_type_from_charactor(cell_def)
            ));
        }
        
        return FieldRow {
            field_cells: cells
        };
    }

    pub fn get_cell_type_from_charactor(character: char) -> i16 {
        return match character {
            ' ' => 0,
            '#' => 2,
            '?' => 3,
            _ => 1
        };
    }
}