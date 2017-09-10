extern crate sdl2;
use sdl2::gfx::primitives::DrawRenderer;
use std;

pub struct CirclePosition {
    pub x: i16,
    pub y: i16,
    pub radius: i16,
    pub color: sdl2::pixels::Color
}

impl CirclePosition {
    pub fn movePosition(&self, renderer: &sdl2::render::Renderer) -> std::result::Result<(), String> {
      return renderer.filled_pie(self.x, self.y, 50, self.radius, 10, self.color);
    }

    pub fn isFullOpenMouth(&mut self) -> bool {
        return 80 <= self.radius;
    }

    pub fn isClosedMouth(&mut self) -> bool {
        return self.radius <= 20;
    }
}