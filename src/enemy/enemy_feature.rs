use sdl2::gfx::primitives::DrawRenderer;
use sdl2::{render, video};

use enemy::enemy_status::{EnemyStatus};

pub trait EnemyAction {
  fn renew(&self, enemy_status: &mut EnemyStatus, renderer: &mut render::Canvas<video::Window>) {
    self.update(enemy_status);
    self.draw(enemy_status, renderer);
  }

  fn update(&self, enemy_status: &mut EnemyStatus) {}

  fn draw(&self, enemy_status: &EnemyStatus, renderer: &mut render::Canvas<video::Window>) {
    let _ = renderer.filled_ellipse(enemy_status.x, enemy_status.y, enemy_status.width, enemy_status.height, enemy_status.background_color);
  }
}

pub struct NormalFeature {}
impl EnemyAction for NormalFeature {}