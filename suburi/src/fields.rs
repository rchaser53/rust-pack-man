extern crate sdl2;

pub struct Field {
    field_rows: Vec<FieldRow>,
}

impl Field {
  // pub fn draw(&self, renderer: &sdl2::render::Canvas<sdl2::video::Window>) -> () {
  pub fn draw(&self){
    let mut i: i8 = 0;
    let mut rows = self.field_rows.iter();

    for row in rows {
      let mut cells = row.field_cells.iter();
      for cell in cells {
        i += 1;
        println!("{}", i);
      }
    }
  }
}

pub struct FieldRow {
    field_cells: Vec<FieldCell>,
}

pub struct FieldCell {
    data: i16
}

pub fn create_field() -> Field {
    let mut rows: Vec<FieldRow> = Vec::new();
    for n in 0 .. 10 {
        rows.push(create_field_row());
    }
    
    return Field {
        field_rows: rows
    };
}

pub fn create_field_row() -> FieldRow {
    let mut cells: Vec<FieldCell> = Vec::new();
    for n in 0 .. 10 {
        let cell = FieldCell {
            data: n
        };
        cells.push(cell);
    }
    
    return FieldRow {
        field_cells: cells
    };
}

pub fn create_field_cell(data: i16) -> FieldCell {
    return FieldCell {
        data: data
    };
}

// rand::thread_rng().gen_range(1, 101);