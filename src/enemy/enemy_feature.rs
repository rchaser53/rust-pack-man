use sdl2::gfx::primitives::DrawRenderer;
use sdl2::{render, video};
use rand;
use rand::Rng;

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

  fn get_random_sign(&self) -> i16 {
    if rand::thread_rng().gen::<bool>() {
      return 1
    }
    -1
  }
}

pub struct NormalFeature {}
impl EnemyAction for NormalFeature {
  fn update(&self, enemy_status: &mut EnemyStatus) {
    enemy_status.x += self.get_random_sign() * rand::thread_rng().gen_range(0, 4);
    enemy_status.y += self.get_random_sign() * rand::thread_rng().gen_range(0, 4);
  }
}