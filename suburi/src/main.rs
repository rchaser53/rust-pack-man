extern crate sdl2;

// #[macro_use]
// extern crate log;
// extern crate env_logger;

use std::{thread, process, time};
use std::path::Path;

use sdl2::video;
use sdl2::event::{Event};
use sdl2::keyboard::Keycode;
// use sdl2::image::{LoadTexture};

pub mod map;
use map::Background;

pub mod constants;
use constants::BackgroundColor::{White, Black, Aqua};

pub mod circle;
use circle::{CirclePosition, Direction};
use Direction::{East, West, South, North};

pub mod collision_handler;
use collision_handler::{CollisionFrame};

pub mod mixer_music;
use mixer_music::{setup_sdl2_mixier, play_bgm, play_sound_effect};

fn create_window(video_ctx: sdl2::VideoSubsystem , width: u32, height: u32) -> video::Window {
    return video_ctx
        .window("Window", width, height)
        .position_centered()
        .opengl()
        .build()
        .unwrap();
}

const SCREEN_WIDTH: i16 = 640;
const SCREEN_HEIGHT: i16 = 640;

fn main() {
    let mut collision_frame = CollisionFrame {
                            screen_width: SCREEN_WIDTH,
                            screen_height: SCREEN_HEIGHT
                        };

    let ctx = sdl2::init().unwrap();
    let video_ctx = ctx.video().unwrap();
    let mut events = ctx.event_pump().unwrap();

    let window = create_window(video_ctx, SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32);

    let mut canvas = window.into_canvas().software().build().unwrap();

    // let texture = texture_creator.load_texture("./hoge.jpg").unwrap();
    let background = Background{
        x: 500, y: 500, border_color: Aqua.value()
    };

    let mut circle_position = CirclePosition{
        x: 300, y:200, direction: East.value(), radius: 30,
        color: White.value(), is_opening_mouth: true
    };

    setup_sdl2_mixier(2);
    let music = play_bgm(Path::new("nyan.mp3"));
    let sound_chunk = play_sound_effect(Path::new("sine.wav"));
    let _ = music.play(1);

    let fifty_millis = time::Duration::from_millis(50);
    let mut main_loop = || {
        thread::sleep(fifty_millis);
        circle_position.move_mouth();

        for event in events.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                    process::exit(1);
                },
                Event::KeyDown { keycode: Some(Keycode::Left), ..} => {
                    circle_position.set_position(-10, 0, West);
                    let _ = sdl2::mixer::Channel::all().play(&sound_chunk, 0);
                },
                Event::KeyDown { keycode: Some(Keycode::Right), ..} => {
                    circle_position.set_position(10, 0, East);
                    let _ = sdl2::mixer::Channel::all().play(&sound_chunk, 0);
                },
                Event::KeyDown { keycode: Some(Keycode::Up), ..} => {
                    circle_position.set_position(0, -10, North);
                    let _ = sdl2::mixer::Channel::all().play(&sound_chunk, 0);
                },
                Event::KeyDown { keycode: Some(Keycode::Down), ..} => {
                    circle_position.set_position(0, 10, South);
                    let _ = sdl2::mixer::Channel::all().play(&sound_chunk, 0);
                },
                _ => {}
            }
        }
        collision_frame.is_out_frame(&circle_position);

        canvas.set_draw_color(Black.value());
        canvas.clear();
        background.draw(&canvas );
        canvas.set_draw_color(White.value());
        circle_position.draw_circle(&canvas);
        canvas.present();
    };

    loop { main_loop(); }
}