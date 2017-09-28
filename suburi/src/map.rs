use sdl2::gfx::primitives::DrawRenderer;
use sdl2::{render, pixels, video};

pub struct Background {
    pub x: i16,
    pub y: i16,
    pub border_color: pixels::Color,
}

struct Hoge {
    x: i16,
    x2: i16,
}

impl Background {
    pub fn draw(&self, renderer: &render::Canvas<video::Window>) -> () {

        let nyan = [Hoge{x:0, x2:30}, Hoge{x:50, x2:130}];

        for i in &nyan {
            let _ = renderer.box_(i.x, 0, i.x2, self.y, self.border_color);
        }

        // let _ = renderer.box_(0, 0, self.x, self.y, self.border_color);
    }
}