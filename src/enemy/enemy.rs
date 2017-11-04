use sdl2::{render, video};

use enemy::enemy_feature::{EnemyAction};
use enemy::enemy_status::{EnemyStatus};

pub struct Enemy {
  pub status: EnemyStatus,
  pub feature: Box<EnemyAction> 
}

impl Enemy {
  pub fn new(status: EnemyStatus, feature: Box<EnemyAction>) -> Enemy {
    Enemy {
      status: status,
      feature: feature
    }
  }

  pub fn draw(&self, renderer: &mut render::Canvas<video::Window>) {
    self.feature.draw(&self.status, renderer);
  }
}