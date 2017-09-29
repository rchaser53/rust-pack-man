use rand::{thread_rng, Rng};

pub struct Field {
    field_rows: Vec<FieldRow>,
}
impl Field {
    pub fn new() -> Field {
        let mut rows: Vec<FieldRow> = Vec::new();
        for n in 0 .. 10 {
            rows.push(FieldRow::new());
        }
        
        return Field {
            field_rows: rows
        };
    }

    // pub fn draw(&self, renderer: &sdl2::render::Canvas<sdl2::video::Window>) -> () {
    pub fn draw(&self){
        let mut rows = self.field_rows.iter();

        for (rowIndex, row) in rows.enumerate() {
            let mut cells = row.field_cells.iter();
            for cell in cells {
               println!("{} in row {}", cell.data, rowIndex);
            }
        }
    }
}

pub struct FieldRow {
    field_cells: Vec<FieldCell>,
}
impl FieldRow {
    pub fn new() -> FieldRow {
        let mut cells: Vec<FieldCell> = Vec::new();
        for n in 0 .. 10 {
            cells.push(FieldCell::new(
                thread_rng().gen_range(1, 3)
            ));
        }
        
        return FieldRow {
            field_cells: cells
        };
    }
}

pub struct FieldCell {
    data: i16
}
impl FieldCell {
    pub fn new(n: i16) -> FieldCell {
        FieldCell {
            data: n
        }
    }
}