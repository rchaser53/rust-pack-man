use std::path::Path;

use sdl2::messagebox;

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

const HIT_EFFECT_PATH: &'static str = "assets/musics/sine.wav";
const HIT_ENEMY_MESSAGE: &'static str = "hit the enemy";
const HIT_ENEMY_WALL: &'static str = "hit the wall";

pub struct CollisionFrame {
  pub screen_width: i16,
  pub screen_height: i16
}
impl CollisionFrame {
  pub fn hit_enemy() -> GameOverError {
    play_sound_effect(Path::new(&HIT_EFFECT_PATH));
    GameOverError::OtherError(HIT_ENEMY_MESSAGE)
  }

  pub fn hit_wall() -> GameOverError {
    play_sound_effect(Path::new(&HIT_EFFECT_PATH));
    GameOverError::OtherError(HIT_ENEMY_WALL)
  }
}