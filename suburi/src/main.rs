extern crate sdl2;
extern crate rand;

// #[macro_use]
// extern crate log;
// extern crate env_logger;

use std::{thread, process, time};
use std::path::Path;

use sdl2::video;
use sdl2::event::{Event};
use sdl2::keyboard::Keycode;
// use sdl2::image::{LoadTexture};

pub mod constants;
use constants::BackgroundColor::{White, Black, Aqua};

pub mod circle;
use circle::{CirclePosition, Direction};
use Direction::{East, West, South, North};

pub mod collision_handler;
use collision_handler::{CollisionFrame};

pub mod mixer_music;
use mixer_music::{setup_sdl2_mixier, play_bgm, play_sound_effect};

pub mod error_handling;
use error_handling::{Result, CustomError};

pub mod fields;
use fields::{Field};

fn create_window(video_ctx: sdl2::VideoSubsystem , width: u32, height: u32)
                    -> Result<video::Window> {
    return video_ctx
                .window("Window", width, height)
                .position_centered()
                .opengl()
                .build()
                .map_err(|err| CustomError::ParseWindowBuildError(err));
}

const SCREEN_WIDTH: i16 = 640;
const SCREEN_HEIGHT: i16 = 640;
const collision_frame: CollisionFrame = CollisionFrame {
                                            screen_width: SCREEN_WIDTH,
                                            screen_height: SCREEN_HEIGHT
                                        };

fn main() {
    // cannnot use fn for const in stable version
    // perhaps i need to try to use nightly version?
    let mut circle_position = CirclePosition::new();
    let fifteen_millis = time::Duration::from_millis(15);

    let ctx = sdl2::init().unwrap_or_else(|err| panic!("{}", err));
    let video_ctx = ctx.video().unwrap();
    let mut events = ctx.event_pump().unwrap();

    let window = create_window(video_ctx, SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32)
                        .unwrap_or_else(|err| panic!("{}", err));
    let mut canvas = window.into_canvas().software().build()
                            .unwrap_or_else(|err| panic!("{}", err));

    setup_sdl2_mixier(2);
    let music = play_bgm(Path::new("nyan.mp3"));
    let _ = music.play(1);

    let mut main_loop = || {
        thread::sleep(fifteen_millis);
        circle_position.move_mouth();
        circle_position.move_circle();

        for event in events.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                    process::exit(1);
                },
                Event::KeyDown { keycode: Some(Keycode::Left), ..} => {
                    circle_position.direction = West.value();
                },
                Event::KeyDown { keycode: Some(Keycode::Right), ..} => {
                    circle_position.direction = East.value();
                },
                Event::KeyDown { keycode: Some(Keycode::Up), ..} => {
                    circle_position.direction = North.value();
                },
                Event::KeyDown { keycode: Some(Keycode::Down), ..} => {
                    circle_position.direction = South.value();
                },
                _ => {}
            }
        }
        collision_frame.is_out_frame(&circle_position);

        canvas.set_draw_color(Black.value());
        canvas.clear();

        canvas.set_draw_color(White.value());
        circle_position.draw_circle(&canvas);
        canvas.present();
    };

    let hoge = Field::new();

    loop { main_loop(); }
}

// fn create_game_screen<'a>() -> Box<FnMut() -> () + 'a> {
//     return Box::new(|| {
//     });
// }

// let texture = texture_creator.load_texture("./hoge.jpg").unwrap();
// let sound_chunk = play_sound_effect(Path::new("sine.wav"));
// let _ = sdl2::mixer::Channel::all().play(&sound_chunk, 0);