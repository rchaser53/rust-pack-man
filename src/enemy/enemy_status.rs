use sdl2::pixels::Color;
use constants::Direction;
use constants::Direction::{East};
use constants::BackgroundColor::{
  Black, White
};

const MOVE_SPEED: i16 = 1;
const ENEMY_WIDTH: i16 = 15;
const ENEMY_HEIGHT: i16 = 15;

pub enum EnemyType {
  Normal, Chaser
}

pub struct UniqueStatus {
  enemy_type: EnemyType,
  background_color: Color
}

pub struct EnemyStatus {
  pub width: i16,
  pub height: i16,
  pub x: i16,
  pub y: i16,
  pub move_speed: i16,
  pub direction: Direction,
  pub enemy_type: EnemyType,
  pub background_color: Color
}

pub trait EnemyStatusFactory {
  fn create_enemy_status(&self, row_index: usize, cell_index: usize) -> EnemyStatus {
    let unique_status = self.create_unique_staus();
    EnemyStatus {
      width: ENEMY_WIDTH,
      height: ENEMY_HEIGHT,
      x: (cell_index * ENEMY_WIDTH as usize) as i16,
      y: (row_index * ENEMY_HEIGHT as usize) as i16,
      move_speed: MOVE_SPEED,
      direction: East,
      background_color: unique_status.background_color,
      enemy_type: unique_status.enemy_type
    }
  }

  fn create_unique_staus(&self) -> UniqueStatus {
    UniqueStatus {
      background_color: White.value(),
      enemy_type: EnemyType::Normal
    }
  }
}

pub struct NormalEnemyStatusFactory {}
impl EnemyStatusFactory for NormalEnemyStatusFactory {}

#[allow(unused)]
pub struct ChaserCellStatusFactory {}
impl EnemyStatusFactory for ChaserCellStatusFactory {
  fn create_unique_staus(&self) -> UniqueStatus {
    UniqueStatus {
      background_color: Black.value(),
      enemy_type: EnemyType::Chaser
    }
  }
}