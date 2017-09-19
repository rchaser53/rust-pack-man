extern crate sdl2;
use sdl2::gfx::primitives::DrawRenderer;

pub struct Background {
    pub x: i16,
    pub y: i16,
    pub border_color: sdl2::pixels::Color,
}

struct Hoge {
    x: i16,
    x2: i16,
}

impl Background {
    pub fn draw(&self, renderer: &sdl2::render::Canvas<sdl2::video::Window>) -> () {

        let nyan = [Hoge{x:0, x2:30}, Hoge{x:50, x2:130}];

        for i in &nyan {
            let _ = renderer.box_(i.x, 0, i.x2, self.y, self.border_color);
        }

        // let _ = renderer.box_(0, 0, self.x, self.y, self.border_color);
    }
}