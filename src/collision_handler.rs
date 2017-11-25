use std::path::Path;

use sdl2::messagebox;

use constants::FILE_PATHS;
use error_handling::{GameOverError};
use mixer_music::play_sound_effect;

pub fn show_message(title: &str, message: &str) {
  let _ = messagebox::show_simple_message_box(
              messagebox::MESSAGEBOX_ERROR,
              title,
              message,
              None
          );
}

const HIT_ENEMY_MESSAGE: &'static str = "hit the enemy";
const HIT_ENEMY_WALL: &'static str = "hit the wall";

pub struct CollisionFrame {
  pub screen_width: i16,
  pub screen_height: i16
}
impl CollisionFrame {
  pub fn hit_enemy() -> GameOverError {
    GameOverError::HitEnemy(HIT_ENEMY_MESSAGE)
  }

  pub fn hit_wall() -> GameOverError {
    GameOverError::OtherError(HIT_ENEMY_WALL)
  }
}