use sdl2::gfx::primitives::DrawRenderer;
use sdl2::{render, pixels, video};
use constants::BackgroundColor::{Aqua};
use constants::Direction::{East, West, South, North};

const MOVE_SPEED: i16 = 5;

pub enum EnemyMode {
  Chase,
  Escape,
  Panic
}

pub struct Enemy {
    pub x: i16,
    pub y: i16,
    pub color: pixels::Color,
    pub mode: EnemyMode
}

impl CirclePosition {
    pub fn new() -> CirclePosition {
        return CirclePosition {
            x: 300, y:200, color: Aqua.value(), mode: EnemyMode::Panic
        };
    }

    pub fn draw(&self, renderer: &render::Canvas<video::Window>) -> () {
        let _ = renderer.filled_ellipse(self.x, self.y, 20, 20, self.color);
    }

    // pub fn move_circle(&mut self) -> () {
    //     self.x += x_amount;
    //     self.y += y_amount;
    // }
}

// pub fn is_full_opened_mouth(&mut self) -> bool {
//     return 80 <= self.radius;
// }

// pub fn is_closed_mouth(&mut self) -> bool {
//     return self.radius <= 10;
// }



// trait DuckLike {
//     fn quack(&self);
//     fn walk(&self) {
//       println!("walking");
//     }
// }

// struct Duck;
// impl DuckLike for Duck {
//     fn quack(&self) {
//         println!("quack");
//     }
// }

// struct Tsuchinoko;
// impl DuckLike for Tsuchinoko {
//     fn quack(&self) {
//         println!("mew");
//     }

//     fn walk(&self) {
//         println!("wriggling");
//     }
// }

// fn duck_go<D: DuckLike>(duck: D) -> D {
//     duck.quack();
//     duck.walk();

//     return duck;
// }

// fn main() {
//     let duck = Duck;
//     let tsuchinoko = Tsuchinoko;

//     duck_go(tsuchinoko);
// }