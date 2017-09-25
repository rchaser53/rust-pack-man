extern crate sdl2;
use sdl2::gfx::primitives::DrawRenderer;

pub enum Direction {
    East,
    South,
    West,
    North
}

impl Direction {
    pub fn value(&self) -> i16 {
        match *self {
            Direction::East => 0,
            Direction::South => 90,
            Direction::West => 180,
            Direction::North => 270
        }
    }
}

const MOVE_SPEED: i16 = 5;

pub struct CirclePosition {
    pub x: i16,
    pub y: i16,
    pub radius: i16,
    pub direction: i16,
    pub color: sdl2::pixels::Color,
    pub is_opening_mouth: bool
}

impl CirclePosition {
    pub fn draw_circle(&self, renderer: &sdl2::render::Canvas<sdl2::video::Window>) -> () {
        let _ = renderer.filled_pie(self.x, self.y, 20, self.radius + self.direction, self.direction, self.color);
    }

    pub fn is_full_opened_mouth(&mut self) -> bool {
        return 80 <= self.radius;
    }

    pub fn is_closed_mouth(&mut self) -> bool {
        return self.radius <= 10;
    }

    pub fn move_mouth(&mut self) -> () {
        if self.is_opening_mouth {
            self.radius += 10;
        } else {
            self.radius -= 10;
        }

        if self.is_full_opened_mouth() || self.is_closed_mouth() {
            self.is_opening_mouth = !self.is_opening_mouth;
        }
    }

    pub fn move_circle(&mut self) -> () {
        let direction = self.direction;
        let (x_amount, y_amount) = match direction {
            num if num == Direction::East.value() => (MOVE_SPEED, 0),
            num if num == Direction::South.value() => (0, MOVE_SPEED),
            num if num == Direction::West.value() => (-1 * MOVE_SPEED, 0),
            num if num == Direction::North.value() => (0, -1 * MOVE_SPEED),
            _ => (0, 0)
        };
        self.x += x_amount;
        self.y += y_amount;
    }
}