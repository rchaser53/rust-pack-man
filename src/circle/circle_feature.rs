use sdl2::{render, rect, pixels, video};

use circle::circle_status::CircleStatus;
use circle::circle::{
  CLOSED_MOUTH_ANGLE, MOVE_SPEED, CIRCLE_RADIUS, FULL_OPENED_MOUTH_ANGLE
};
use constants::Direction::{
  East, South, West, North
};

pub struct CircleFeature {}
impl CircleFeature {
    pub fn new() -> CircleFeature {
      CircleFeature{}
    }

    pub fn renew(&self, circle_status: &mut CircleStatus, renderer: &mut render::Canvas<video::Window>) {
        self.update_animation_model(circle_status);
        self.draw(circle_status, renderer);
    }

    pub fn update_animation_model(&self, circle_status: &mut CircleStatus) {
        self.move_mouth(circle_status);

        if circle_status.is_stoped { return; }
        self.move_circle(circle_status);
    }

    pub fn move_mouth(&self, circle_status: &mut CircleStatus) {
        if circle_status.is_opening_mouth {
            circle_status.radius += CLOSED_MOUTH_ANGLE;
        } else {
            circle_status.radius -= CLOSED_MOUTH_ANGLE;
        }

        if self.is_full_opened_mouth(circle_status) || self.is_closed_mouth(circle_status) {
            circle_status.is_opening_mouth = !circle_status.is_opening_mouth;
        }
    }

    pub fn move_circle(&self, circle_status: &mut CircleStatus) {
        let (x_amount, y_amount) = match circle_status.direction {
            East => (MOVE_SPEED, 0),
            South => (0, MOVE_SPEED),
            West => (-MOVE_SPEED, 0),
            North => (0, -MOVE_SPEED),
        };
        circle_status.hitbox.x += x_amount;
        circle_status.hitbox.y += y_amount;
    }

    pub fn is_full_opened_mouth(&self, circle_status: &mut CircleStatus) -> bool {
        FULL_OPENED_MOUTH_ANGLE <= circle_status.radius
    }

    pub fn is_closed_mouth(&self, circle_status: &mut CircleStatus) -> bool {
        circle_status.radius <= CLOSED_MOUTH_ANGLE
    }

    pub fn draw(&self, circle_status: &mut CircleStatus, renderer: &mut render::Canvas<video::Window>) {
        renderer.set_draw_color(circle_status.color as pixels::Color);

        let rect = rect::Rect::new(circle_status.hitbox.x as i32 - 10, circle_status.hitbox.y  as i32 - 10, 20, 20);
        let _ = renderer.fill_rect(rect);
    }
}