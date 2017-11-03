use game_field::field_cell::{FieldCell};
use game_field::cell_status::{CellStatus};
use constants::BackgroundColor;
use game_field::cell_feature::{
    NormalFeature
};
//     DrawMyself,
    
//     DamageFeature,
//     BlockFeature,
//     WallFeature,
//     ItemFeature


pub struct FieldRow {
    pub field_cells: Vec<FieldCell>
}
impl FieldRow {
    pub fn new(row_def: &str, row_index: usize) -> FieldRow {
        let mut cells: Vec<FieldCell> = Vec::new();

        let cell_defs: Vec<char> = row_def.chars().collect();
        let cell_defs_length = cell_defs.len();
        for cell_index in 0..cell_defs_length {
            cells.push(FieldCell::new(
                CellStatus::new(BackgroundColor::Black.value(), row_index, cell_index),
                Box::new(NormalFeature {})
            ));
        }

        FieldRow {
            field_cells: cells
        }
    }
}

    // FieldRow::get_cell_type_from_charactor(cell_defs[cell_index])

    // pub fn get_cell_type_from_charactor(character: char) -> CellFeature {
    //     match character {
    //         ' ' => CellFeature::new(BackgroundColor::Black.value()),
    //         '#' => CellFeature::new(BackgroundColor::Black.value()),
    //         '?' => CellFeature::new(BackgroundColor::Black.value()),
    //         '*' => CellFeature::new(BackgroundColor::Black.value()),
    //         _ => CellFeature::new(BackgroundColor::Black.value()),
    //     }
    // }

    // ' ' => 0,
    // '#' => 2,
    // '?' => 3,
    // '*' => 4,
    // _ => 1

    // CellType::Normal => BackgroundColor::Black.value(),
    // CellType::Block => BackgroundColor::Aqua.value(),
    // CellType::Damage => BackgroundColor::Aqua.value(),
    // CellType::Wall => BackgroundColor::Gray.value(),
    // CellType::Item => BackgroundColor::Black.value()