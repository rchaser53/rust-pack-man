extern crate sdl2;

#[macro_use]
extern crate log;
extern crate env_logger;

use std::{thread, process, time};
use std::path::Path;

use sdl2::video;
use sdl2::event::{Event};
use sdl2::keyboard::Keycode;
use sdl2::image::{LoadTexture, INIT_PNG, INIT_JPG};

pub mod constants;
use constants::BackgroundColor::{white, black};

pub mod deviceMusic;
use deviceMusic::createDeviceMusic;

pub mod circle;
use circle::{CirclePosition, Direction};

fn createWindow(video_ctx: sdl2::VideoSubsystem , width: u32, height: u32) -> video::Window {
    return video_ctx
        .window("Window", width, height)
        .position_centered()
        .opengl()
        .build()
        .unwrap();
}

fn main() {
    // env_logger::init();
    // info!("starting up");

    let ctx = sdl2::init().unwrap();
    let video_ctx = ctx.video().unwrap();
    let mut events = ctx.event_pump().unwrap();

    let window = createWindow(video_ctx, 640, 640);

    let mut canvas = window.into_canvas().software().build().unwrap();

    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture("./hoge.jpg").unwrap();

    let mut circlePosition = CirclePosition{
        x: 300, y:200, direction: Direction::east.value(), radius: 30,
        color: white.value(), isOpeningMouth: true
    };

    let path: &'static Path = Path::new("./sine.wav");
    let mut device = createDeviceMusic(&ctx, path);

    let fifty_millis = time::Duration::from_millis(50);
    let mut main_loop = || {
        thread::sleep(fifty_millis);
        circlePosition.moveMouth();

        for event in events.poll_iter() {
            device.pause();
            match event {
                Event::Quit {..} => {
                    process::exit(1);
                },
                Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                    process::exit(1);
                },
                Event::KeyDown { keycode: Some(Keycode::Left), ..} => {
                    circlePosition.x -= 10;
                    circlePosition.direction = Direction::west.value();
                    device.resume();
                },
                Event::KeyDown { keycode: Some(Keycode::Right), ..} => {
                    circlePosition.x += 10;
                    circlePosition.direction = Direction::east.value();
                    device.resume();
                },
                Event::KeyDown { keycode: Some(Keycode::Up), ..} => {
                    circlePosition.y -= 10;
                    circlePosition.direction = Direction::north.value();
                    device.resume();
                },
                Event::KeyDown { keycode: Some(Keycode::Down), ..} => {
                    circlePosition.y += 10;
                    circlePosition.direction = Direction::south.value();
                    device.resume();
                },
                _ => {}
            }
        }
        canvas.set_draw_color(black.value());
        canvas.clear();
        canvas.copy(&texture, None, None).expect("Background Image Render failed");
        canvas.set_draw_color(white.value());
        circlePosition.movePosition(&canvas);
        canvas.present();
    };

    loop { main_loop(); }
}