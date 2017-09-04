extern crate sdl2;

use std::process;
use sdl2::video;
use sdl2::rect::{Rect};
use sdl2::event::{Event};
use sdl2::keyboard::Keycode;

pub mod messagebox;
use messagebox::showMessage;

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

    let mut main_loop = || {
        for event in events.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                    println!("nya-n");
                    process::exit(1);
                },
                Event::KeyDown { keycode: Some(Keycode::Left), ..} => {
                    showMessage("nyanTitle", "pyaaMessage");
                },
                Event::KeyDown { keycode: Some(Keycode::Right), ..} => {
                    rect.x += 10;
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
        let _ = renderer.present();
    };

    loop { main_loop(); }
}