use sdl2::messagebox;
use circle::CirclePosition;
use std::process;

pub fn show_message(title: &str, message: &str) -> () {
  let _ = messagebox::show_simple_message_box(
              messagebox::MESSAGEBOX_ERROR,
              title,
              message,
              None
          );
}

pub struct CollisionFrame {
  pub screen_width: i16,
  pub screen_height: i16
}

impl CollisionFrame {
  pub fn is_out_frame(&mut self, circle: &CirclePosition) -> () {
    if self.is_out_xaxis(&circle) || self.is_out_yaxis(&circle) {
      let _ = show_message("title", "out screen!");
      process::exit(1);
    }
  }

  pub fn is_out_xaxis(&mut self, circle: &CirclePosition) -> bool {
    return circle.x < 0 || self.screen_width < circle.x;
  }

  pub fn is_out_yaxis(&mut self, circle: &CirclePosition) -> bool {
    return circle.y < 0 || self.screen_height < circle.y;
  }
}