use sdl2::gfx::primitives::DrawRenderer;
use sdl2::{render, pixels, video};
use constants::BackgroundColor::{White};
use constants::Direction::{East, West, South, North};

const MOVE_SPEED: i16 = 5;
const INITIAL_X: i16 = 50;
const INITIAL_Y: i16 = 50;
const CIRCLE_RADIUS: i16 = 15;
const FULL_OPENED_MOUTH_ANGLE: i16 = 80;
const CLOSED_MOUTH_ANGLE: i16 = 10;

pub struct Circle {
    pub x: i16,
    pub y: i16,
    pub radius: i16,
    pub direction: i16,
    pub color: pixels::Color,
    pub is_opening_mouth: bool
}

impl Circle {
    pub fn new() -> Circle {
        return Circle {
            x: INITIAL_X, y: INITIAL_Y, direction: East.value(),
            radius: CIRCLE_RADIUS, color: White.value(), is_opening_mouth: true
        };
    }

    pub fn renew(&mut self, renderer: &mut render::Canvas<video::Window>) -> () {
        self.update_animation_model();
        self.draw(renderer);
    }

    pub fn update_animation_model(&mut self) -> () {
        self.move_mouth();
        self.move_circle();
    }

    pub fn move_mouth(&mut self) -> () {
        if self.is_opening_mouth {
            self.radius += CLOSED_MOUTH_ANGLE;
        } else {
            self.radius -= CLOSED_MOUTH_ANGLE;
        }

        if self.is_full_opened_mouth() || self.is_closed_mouth() {
            self.is_opening_mouth = !self.is_opening_mouth;
        }
    }

    pub fn move_circle(&mut self) -> () {
        let direction = self.direction;
        let (x_amount, y_amount) = match direction {
            num if num == East.value() => (MOVE_SPEED, 0),
            num if num == South.value() => (0, MOVE_SPEED),
            num if num == West.value() => (-1 * MOVE_SPEED, 0),
            num if num == North.value() => (0, -1 * MOVE_SPEED),
            _ => (0, 0)
        };
        self.x += x_amount;
        self.y += y_amount;
    }

    pub fn is_full_opened_mouth(&mut self) -> bool {
        return FULL_OPENED_MOUTH_ANGLE <= self.radius;
    }

    pub fn is_closed_mouth(&mut self) -> bool {
        return self.radius <= CLOSED_MOUTH_ANGLE;
    }

    pub fn draw(&mut self, renderer: &mut render::Canvas<video::Window>) -> () {
        let _ = renderer.filled_pie(self.x, self.y, CIRCLE_RADIUS,
                                    self.radius + self.direction, self.direction, self.color);
    }
}