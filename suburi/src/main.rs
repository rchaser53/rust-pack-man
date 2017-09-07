extern crate sdl2;

use std::{thread, time};
use std::process;
use sdl2::video;
use sdl2::rect::{Rect};
use sdl2::event::{Event};
use sdl2::keyboard::Keycode;
use sdl2::gfx::primitives::DrawRenderer;

pub mod messagebox;
use messagebox::showMessage;

struct CirclePosition {
    x: i16,
    y: i16,
    radius: i16,
    color: sdl2::pixels::Color
}

impl CirclePosition {
    pub fn movePosition(&self, renderer: &sdl2::render::Renderer) -> std::result::Result<(), String> {
      return renderer.filled_pie(self.x, self.y, 50, self.radius, 0, self.color);
    }

    pub fn setX(&mut self, x: i16) -> () {
        self.x = x;
    }

    pub fn addRadius(&mut self, radius: i16) -> () {
        self.radius += radius;
    }
}

fn createWindow(video_ctx: sdl2::VideoSubsystem , width: u32, height: u32) -> video::Window {
    return match video_ctx
        .window("rust_to_js", width, height)
        .position_centered()
        .opengl()
        .build() {
            Ok(window) => window,
            Err(err)   => panic!("failed to create window: {}", err)
        };
}

fn main() {
    let ctx = sdl2::init().unwrap();
    let video_ctx = ctx.video().unwrap();
    let window = createWindow(video_ctx, 640, 640);

    let mut renderer = match window
                                .renderer()
                                .build() {
                                    Ok(renderer) => renderer,
                                    Err(err) => panic!("failed to create renderer: {}", err)
                                };

    let mut rect = Rect::new(10, 10, 10, 10);

    let black = sdl2::pixels::Color::RGB(0, 0, 0);
    let white = sdl2::pixels::Color::RGB(255, 255, 255);

    let mut events = ctx.event_pump().unwrap();

    let mut circlePosition = CirclePosition{
        x: 300, y:200, color: white, radius: 0
    };

    let fifty_millis = time::Duration::from_millis(50);

    let mut main_loop = || {
        thread::sleep(fifty_millis);
        let _ = circlePosition.addRadius(10);
        for event in events.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                    println!("nya-n");
                    process::exit(1);
                },
                Event::KeyDown { keycode: Some(Keycode::Left), ..} => {
                    let _ = circlePosition.setX(100);
                },
                Event::KeyDown { keycode: Some(Keycode::Right), ..} => {
                    let _ = circlePosition.setX(500);
                },
                Event::KeyDown { keycode: Some(Keycode::Up), ..} => {
                    rect.y -= 10;
                },
                Event::KeyDown { keycode: Some(Keycode::Down), ..} => {
                    rect.y += 10;
                },
                _ => {}
            }
        }

        let _ = renderer.set_draw_color(black);
        let _ = renderer.clear();
        let _ = renderer.set_draw_color(white);
        let _ = renderer.fill_rect(rect);
        let _ = circlePosition.movePosition(&renderer);
        let _ = renderer.present();
    };

    loop { main_loop(); }
    loop { }
}