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
use constants::Direction::{East, West, South, North};

pub mod circle;
use circle::{CirclePosition};

pub mod collision_handler;
use collision_handler::{CollisionFrame};

pub mod mixer_music;
use mixer_music::{setup_sdl2_mixier, play_bgm, play_sound_effect};

pub mod error_handling;
use error_handling::{Result, CustomError};

pub mod fields;
use fields::{Field, SCREEN_WIDTH, SCREEN_HEIGHT};

trait CanvasBackground {
    fn setup_draw_background(&mut self) -> ();
}
impl CanvasBackground for sdl2::render::WindowCanvas {
    fn setup_draw_background(&mut self) -> () {
        self.set_draw_color(Black.value());
        self.clear();
        self.set_draw_color(White.value());
    }
}

fn create_window(video_ctx: sdl2::VideoSubsystem , width: u32, height: u32)
                    -> Result<video::Window> {
    return video_ctx
                .window("Window", width, height)
                .position_centered()
                .opengl()
                .build()
                .map_err(|err| CustomError::ParseWindowBuildError(err));
}

const collision_frame: CollisionFrame = CollisionFrame {
                                            screen_width: SCREEN_WIDTH,
                                            screen_height: SCREEN_HEIGHT
                                        };

fn main() {
    // cannnot use fn for const in stable version
    // perhaps i need to try to use nightly version?
    let mut circle = CirclePosition::new();
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

    let hoge = Field::new();

    let mut main_loop = || {
        thread::sleep(fifteen_millis);

        for event in events.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                    process::exit(1);
                },
                Event::KeyDown { keycode: Some(Keycode::Left), ..} => {
                    circle.direction = West.value();
                },
                Event::KeyDown { keycode: Some(Keycode::Right), ..} => {
                    circle.direction = East.value();
                },
                Event::KeyDown { keycode: Some(Keycode::Up), ..} => {
                    circle.direction = North.value();
                },
                Event::KeyDown { keycode: Some(Keycode::Down), ..} => {
                    circle.direction = South.value();
                },
                _ => {}
            }
        }
        // hoge.draw(&mut canvas);
        collision_frame.is_out_frame(&circle);

        canvas.setup_draw_background();
        circle.draw(&canvas);
        canvas.present();
    };


    loop { main_loop(); }
}

// fn create_game_screen<'a>() -> Box<FnMut() -> () + 'a> {
//     return Box::new(|| {
//     });
// }

// let texture = texture_creator.load_texture("./hoge.jpg").unwrap();
// let sound_chunk = play_sound_effect(Path::new("sine.wav"));
// let _ = sdl2::mixer::Channel::all().play(&sound_chunk, 0);