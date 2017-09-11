extern crate sdl2;

use std::{thread, time};
use std::process;
use sdl2::video;
use sdl2::rect::{Rect};
use sdl2::event::{Event};
use sdl2::keyboard::Keycode;
use sdl2::gfx::primitives::DrawRenderer;

use sdl2::image::{LoadTexture, INIT_PNG, INIT_JPG};

pub mod messagebox;
use messagebox::showMessage;

// pub mod circle;
// use circle::CirclePosition;

fn createWindow(video_ctx: sdl2::VideoSubsystem , width: u32, height: u32) -> video::Window {
    return video_ctx
        .window("Window", width, height)
        .position_centered()
        .opengl()
        .build()
        .unwrap();
}

fn main() {
    let ctx = sdl2::init().unwrap();
    let video_ctx = ctx.video().unwrap();
    let _image_context = sdl2::image::init(INIT_PNG | INIT_JPG).unwrap();

    let window = createWindow(video_ctx, 640, 640);

    // let window = video_ctx.window("Window", 800, 600)
    //                     .opengl()
    //                     .build()
    //                     .unwrap();


    let mut canvas = window.into_canvas().software().build().unwrap();
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture("./hoge.jpg").unwrap();
 
    // canvas.copy(&texture, None, None).expect("Render failed");
    // canvas.present();


    // let mut renderer = match window
    //                             .renderer()
    //                             .build() {
    //                                 Ok(renderer) => renderer,
    //                                 Err(err) => panic!("failed to create renderer: {}", err)
    //                             };

    // let mut canvas = window.into_canvas().software().build().unwrap();
    // let texture_creator = canvas.texture_creator();
    // let texture = texture_creator.load_texture(png).unwrap();


    let black = sdl2::pixels::Color::RGB(0, 0, 0);
    let white = sdl2::pixels::Color::RGB(255, 255, 255);

    let mut events = ctx.event_pump().unwrap();

    // let mut circlePosition = CirclePosition{
    //     x: 300, y:200, color: white,
    //     radius: 30, isOpeningMouth: true
    // };
    let fifty_millis = time::Duration::from_millis(50);

    let mut main_loop = || {
        thread::sleep(fifty_millis);
        // circlePosition.moveMouth();

        for event in events.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                    process::exit(1);
                },
                // Event::KeyDown { keycode: Some(Keycode::Left), ..} => {
                //     circlePosition.x -= 10;
                // },
                // Event::KeyDown { keycode: Some(Keycode::Right), ..} => {
                //     circlePosition.x += 10;
                // },
                // Event::KeyDown { keycode: Some(Keycode::Up), ..} => {
                //     circlePosition.y -= 10;
                // },
                // Event::KeyDown { keycode: Some(Keycode::Down), ..} => {
                //     circlePosition.y += 10;
                // },
                _ => {}
            }
        }
        let _ = canvas.set_draw_color(black);
        let _ = canvas.clear();
        let _ = canvas.set_draw_color(white);
        // let _ = circlePosition.movePosition(&renderer);
        canvas.copy(&texture, None, None).expect("Render failed");
        let _ = canvas.present();
    };

    loop { main_loop(); }
}