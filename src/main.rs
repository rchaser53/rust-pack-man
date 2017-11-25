#[macro_use]
extern crate enum_primitive;
#[macro_use]
extern crate lazy_static;
extern crate num;
extern crate sdl2;
extern crate rand;

#[cfg(target_os = "emscripten")]
pub mod emscripten;

use std::{thread, process, time};
use std::path::Path;

use sdl2::video;
use sdl2::EventPump;
use sdl2::event::{Event};
use sdl2::keyboard::Keycode;
// use sdl2::image::{LoadTexture};

pub mod constants;
use constants::{FILE_PATHS, BackgroundColor, Direction};
use BackgroundColor::{White, Black};
use Direction::{East, West, South, North};

pub mod hitbox;
pub mod mixer_music;
use mixer_music::{setup_sdl2_mixier, play_bgm};

pub mod error_handling;
use error_handling::{Result, CustomError};

pub mod game_status;
pub mod circle;

mod game_field;
use game_field::field::{Field};
use game_field::position_handler::{SCREEN_WIDTH, SCREEN_HEIGHT};

mod enemy;

trait CanvasBackground {
    fn setup_draw_background(&mut self);
}
impl CanvasBackground for sdl2::render::WindowCanvas {
    fn setup_draw_background(&mut self) {
        self.set_draw_color(Black.value());
        self.clear();
        self.set_draw_color(White.value());
    }
}

fn create_window(video_ctx: sdl2::VideoSubsystem , width: u32, height: u32)
                    -> Result<video::Window> {
    video_ctx
         .window("Window", width, height)
         .position_centered()
         .opengl()
         .build()
         .map_err(CustomError::ParseWindowBuildError)
}

fn handle_event(events: &mut EventPump, field: &mut Field) {
    for event in events.poll_iter() {
        match event {
            Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                process::exit(1);
            },
            Event::KeyDown { keycode: Some(Keycode::Left), ..}
                | Event::KeyDown { keycode: Some(Keycode::H), ..} => {
                field.circle.status.direction = West;
            },
            Event::KeyDown { keycode: Some(Keycode::Right), ..}
                | Event::KeyDown { keycode: Some(Keycode::L), ..} => {
                field.circle.status.direction = East;
            },
            Event::KeyDown { keycode: Some(Keycode::Up), ..}
                | Event::KeyDown { keycode: Some(Keycode::K), ..} => {
                field.circle.status.direction = North;
            },
            Event::KeyDown { keycode: Some(Keycode::Down), ..}
                | Event::KeyDown { keycode: Some(Keycode::J), ..} => {
                field.circle.status.direction = South;
            },
            Event::KeyDown {keycode: Some(Keycode::Space), ..} => {
                field.game_status.is_pause = !field.game_status.is_pause;
            },
            _ => {}
        }
    }
}

fn main() {
    // cannnot use fn for const in stable version
    // perhaps i need to try to use nightly version?
    let fifteen_millis = time::Duration::from_millis(15);

    let ctx = sdl2::init().unwrap_or_else(|err| panic!("{}", err));
    let video_ctx = ctx.video().unwrap();
    let mut events = ctx.event_pump().unwrap();

    let window = create_window(video_ctx, SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32)
                        .unwrap_or_else(|err| panic!("{}", err));
    let mut canvas = window.into_canvas().software().build()
                            .unwrap_or_else(|err| panic!("{}", err));

    setup_sdl2_mixier(2);
    let music = play_bgm(Path::new(FILE_PATHS.get("BGM_PATH").unwrap()));
    let _ = music.play(1);

    let mut field = Field::new();

    let mut main_loop = || {
        thread::sleep(fifteen_millis);
        handle_event(&mut events, &mut field);

        canvas.setup_draw_background();
        let _ = field.renew(&mut canvas).map_err(|err| {
            println!("{}", err)
        });
        canvas.present();
    };

    #[cfg(target_os = "emscripten")]
    use emscripten::{emscripten};

    #[cfg(target_os = "emscripten")]
    emscripten::set_main_loop_callback(main_loop);

    #[cfg(not(target_os = "emscripten"))]
    loop { main_loop(); }
}