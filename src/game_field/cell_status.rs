use sdl2::pixels::Color;

use constants::{CellType, CELL_WIDTH, CELL_HEIGHT};
use constants::CellType::{
  Normal, Block, Damage, Wall, Item
};
use constants::BackgroundColor::{
  Black, Aqua, White, Gray
};

pub struct CellStatus {
  pub width: u32,
  pub height: u32,
  pub x: i32,
  pub y: i32,
  pub cell_type: CellType,
  pub background_color: Color,
  pub exist_item: bool
}

pub struct UniqueStatus {
  background_color: Color,
  cell_type: CellType,
  exist_item: bool
}

pub trait CellStatusFactory {
  fn create_cell_status(&self, row_index: usize, cell_index: usize) -> CellStatus {
    let unique_status = self.create_unique_staus();
    CellStatus {
      x: (cell_index * CELL_WIDTH as usize) as i32,
      y: (row_index * CELL_HEIGHT as usize) as i32,
      width: CELL_WIDTH as u32,
      height: CELL_HEIGHT as u32,
      background_color: unique_status.background_color,
      cell_type: unique_status.cell_type,
      exist_item: unique_status.exist_item
    }
  }

  fn create_unique_staus(&self) -> UniqueStatus {
    UniqueStatus {
      background_color: Black.value(),
      cell_type: Normal,
      exist_item: false
    }
  }
}

pub struct NormalCellStatusFactory {}
impl CellStatusFactory for NormalCellStatusFactory {}

pub struct BlockCellStatusFactory {}
impl CellStatusFactory for BlockCellStatusFactory {
  fn create_unique_staus(&self) -> UniqueStatus {
    UniqueStatus {
      background_color: Aqua.value(),
      cell_type: Block,
      exist_item: false
    }
  }
}

#[allow(unused)]
pub struct DamageCellStatusFactory {}
impl CellStatusFactory for DamageCellStatusFactory {
  fn create_unique_staus(&self) -> UniqueStatus {
    UniqueStatus {
      background_color: White.value(),
      cell_type: Damage,
      exist_item: false
    }
  }
}

pub struct WallCellStatusFactory {}
impl CellStatusFactory for WallCellStatusFactory {
  fn create_unique_staus(&self) -> UniqueStatus {
    UniqueStatus {
      background_color: Gray.value(),
      cell_type: Wall,
      exist_item: false
    }
  }
}

pub struct ItemCellStatusFactory {}
impl CellStatusFactory for ItemCellStatusFactory {
  fn create_unique_staus(&self) -> UniqueStatus {
    UniqueStatus {
      background_color: Black.value(),
      cell_type: Item,
      exist_item: true
    }
  }
}