#[macro_use]
extern crate enum_primitive;
extern crate num;
extern crate sdl2;
extern crate rand;

// #[macro_use]
// extern crate log;
// extern crate env_logger;

use std::{thread, process, time};
use std::path::Path;

use sdl2::video;
use sdl2::EventPump;
use sdl2::event::{Event};
use sdl2::keyboard::Keycode;
// use sdl2::image::{LoadTexture};

pub mod constants;
use constants::BackgroundColor::{White, Black};
use constants::Direction::{East, West, South, North};

pub mod circle;
use circle::{Circle};

pub mod mixer_music;
use mixer_music::{setup_sdl2_mixier, play_bgm};

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

fn handle_event(events: &mut EventPump, circle: &mut Circle) -> () {
    for event in events.poll_iter() {
        match event {
            Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                process::exit(1);
            },
            Event::KeyDown { keycode: Some(Keycode::Left), ..}
                | Event::KeyDown { keycode: Some(Keycode::H), ..} => {
                circle.direction = West.value();
            },
            Event::KeyDown { keycode: Some(Keycode::Right), ..}
                | Event::KeyDown { keycode: Some(Keycode::L), ..} => {
                circle.direction = East.value();
            },
            Event::KeyDown { keycode: Some(Keycode::Up), ..}
                | Event::KeyDown { keycode: Some(Keycode::K), ..} => {
                circle.direction = North.value();
            },
            Event::KeyDown { keycode: Some(Keycode::Down), ..}
                | Event::KeyDown { keycode: Some(Keycode::J), ..} => {
                circle.direction = South.value();
            },
            _ => {}
        }
    }
}

const BGM_PATH: &'static str = "assets/musics/nyan.mp3";

fn main() {
    // cannnot use fn for const in stable version
    // perhaps i need to try to use nightly version?
    let mut circle = Circle::new();
    let fifteen_millis = time::Duration::from_millis(15);

    let ctx = sdl2::init().unwrap_or_else(|err| panic!("{}", err));
    let video_ctx = ctx.video().unwrap();
    let mut events = ctx.event_pump().unwrap();

    let window = create_window(video_ctx, SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32)
                        .unwrap_or_else(|err| panic!("{}", err));
    let mut canvas = window.into_canvas().software().build()
                            .unwrap_or_else(|err| panic!("{}", err));

    setup_sdl2_mixier(2);
    let music = play_bgm(Path::new(BGM_PATH));
    let _ = music.play(1);

    let mut field = Field::new(&mut circle);

    let mut main_loop = || {
        thread::sleep(fifteen_millis);
        handle_event(&mut events, &mut field.circle);

        canvas.setup_draw_background();
        let _ = field.renew(&mut canvas).map_err(|err| println!("{}", err));
        canvas.present();
    };

    loop { main_loop(); }
}