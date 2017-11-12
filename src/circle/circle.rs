use circle::circle_feature::CircleFeature;
use circle::circle_status::CircleStatus;

pub const INITIAL_X: i16 = 50;
pub const INITIAL_Y: i16 = 50;
pub const CIRCLE_RADIUS: i16 = 15;
pub const MOVE_SPEED: i16 = 5;
pub const FULL_OPENED_MOUTH_ANGLE: i16 = 80;
pub const CLOSED_MOUTH_ANGLE: i16 = 10;

pub struct Circle {
    pub status: CircleStatus,
    pub feature: CircleFeature
}

impl Circle {
    pub fn new() -> Circle {
        Circle {
            status: CircleStatus::new(),
            feature: CircleFeature::new()
        }
    }
}