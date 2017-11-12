use sdl2::{pixels};

use constants::BackgroundColor::{White};
use constants::Direction::{East};
use constants::Direction;
use hitbox::Hitbox;

use circle::circle::{
  INITIAL_X, INITIAL_Y, CIRCLE_RADIUS
};

pub struct CircleStatus {
    pub hitbox: Hitbox,
    pub radius: i16,
    pub direction: Direction,
    pub color: pixels::Color,
    pub is_opening_mouth: bool,
    pub is_stoped: bool
}

impl CircleStatus {
    pub fn new() -> CircleStatus {
        let hitbox = Hitbox {
            x: INITIAL_X, y: INITIAL_Y,
            width: CIRCLE_RADIUS, height: CIRCLE_RADIUS
        };
        CircleStatus {
            hitbox: hitbox, radius: CIRCLE_RADIUS, direction: East,
            color: White.value(), is_opening_mouth: true, is_stoped: false
        }
    }
}