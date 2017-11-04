use game_field::field_cell::{FieldCell};
use game_field::cell_status::{
    CellStatus,
    CellStatusFactory,
    NormalCellStatusFactory,
    BlockCellStatusFactory,
    WallCellStatusFactory,
    ItemCellStatusFactory
};
use game_field::cell_feature::{
    DrawMyself,
    NormalFeature,
    BlockFeature,
    WallFeature,
    ItemFeature 
};

pub struct FieldRow {
    pub field_cells: Vec<FieldCell>
}
impl FieldRow {
    pub fn new(row_def: &str, row_index: usize) -> FieldRow {
        let mut cells: Vec<FieldCell> = Vec::new();

        let cell_defs: Vec<char> = row_def.chars().collect();
        let cell_defs_length = cell_defs.len();
        for cell_index in 0..cell_defs_length {
            let (cell_status, cell_feature) = FieldRow::get_cell_data(cell_defs[cell_index], row_index, cell_index);
            cells.push(FieldCell::new(cell_status, cell_feature));
        }

        FieldRow {
            field_cells: cells
        }
    }

    pub fn get_cell_data(character: char, row_index: usize, cell_index: usize) -> (CellStatus, Box<DrawMyself>) {
        match character {
            ' ' => (Box::new(NormalCellStatusFactory{}).create_cell_status(row_index, cell_index), Box::new(NormalFeature {})),
            '#' => (Box::new(BlockCellStatusFactory{}).create_cell_status(row_index, cell_index), Box::new(BlockFeature {})),
            '?' => (Box::new(WallCellStatusFactory{}).create_cell_status(row_index, cell_index), Box::new(WallFeature {})),
            '*' => (Box::new(ItemCellStatusFactory{}).create_cell_status(row_index, cell_index), Box::new(ItemFeature {})),
            _ => (Box::new(NormalCellStatusFactory{}).create_cell_status(row_index, cell_index), Box::new(NormalFeature {}))
        }
    }
}