use sdl2::gfx::primitives::DrawRenderer;
use sdl2::{render, video};
use rand;
use rand::Rng;

use game_field::field::Field;
use circle::Circle;
use enemy::enemy_status::{EnemyStatus};

pub trait EnemyAction {
  fn renew(&self, field: &Field, enemy_status: &mut EnemyStatus, renderer: &mut render::Canvas<video::Window>) {
    self.update(&field, enemy_status);
    self.draw(enemy_status, renderer);
  }

  fn update(&self, field: &Field, enemy_status: &mut EnemyStatus) {}

  fn draw(&self, enemy_status: &EnemyStatus, renderer: &mut render::Canvas<video::Window>) {
    let _ = renderer.filled_ellipse(enemy_status.x, enemy_status.y, enemy_status.width, enemy_status.height, enemy_status.background_color);
  }

  fn get_direction(&self, circle_position: i16, enemy_position: i16) -> i16 {
    if 0 < (circle_position - enemy_position) {
      return 1
    }
    -1
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
  fn update(&self, field: &Field, enemy_status: &mut EnemyStatus) {
    let circle = &field.circle;

    enemy_status.x += self.get_direction(circle.x, enemy_status.x) * rand::thread_rng().gen_range(0, 4);
    enemy_status.y += self.get_direction(circle.y, enemy_status.y) * rand::thread_rng().gen_range(0, 4);
  }
}