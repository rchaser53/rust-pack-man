extern crate sdl2;

use std::{thread, process, time};
use std::path::{PathBuf, Path};
use std::borrow::Cow;

use sdl2::video;
use sdl2::rect::{Rect};
use sdl2::event::{Event};
use sdl2::keyboard::Keycode;
use sdl2::gfx::primitives::DrawRenderer;

use sdl2::image::{LoadTexture, INIT_PNG, INIT_JPG};

use sdl2::audio::{AudioCallback, AudioSpecDesired, AudioSpecWAV, AudioCVT};

pub mod circle;
use circle::{CirclePosition, Direction};

struct Sound {
    data: Vec<u8>,
    volume: f32,
    pos: usize,
}

impl AudioCallback for Sound {
    type Channel = u8;

    fn callback(&mut self, out: &mut [u8]) {
        for dst in out.iter_mut() {
            *dst = (*self.data.get(self.pos).unwrap_or(&0) as f32 * self.volume) as u8;
            self.pos += 1;
        }
    }
}

fn createDeviceMusic(audio_subsystem: &sdl2::AudioSubsystem) -> sdl2::audio::AudioDevice<Sound> {
    let wav_file : Cow<'static, Path> = match std::env::args().nth(1) {
        None => Cow::from(Path::new("./sine.wav")),
        Some(s) => Cow::from(PathBuf::from(s))
    };

    let desired_spec = AudioSpecDesired {
        freq: Some(44100),
        channels: Some(1), // mono
        samples: None      // default
    };

    return audio_subsystem.open_playback(None, &desired_spec, |spec| {
        let wav = AudioSpecWAV::load_wav(wav_file)
            .expect("Could not load test WAV file");

        let cvt = AudioCVT::new(
                wav.format, wav.channels, wav.freq,
                spec.format, spec.channels, spec.freq)
            .expect("Could not convert WAV file");

        let data = cvt.convert(wav.buffer().to_vec());

        // initialize the audio callback
        Sound {
            data: data,
            volume: 0.25,
            pos: 0,
        }
    }).unwrap();
}

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
    let audio_subsystem = ctx.audio().unwrap();
    let _image_context = sdl2::image::init(INIT_PNG | INIT_JPG).unwrap();

    let window = createWindow(video_ctx, 640, 640);

    let mut canvas = window.into_canvas().software().build().unwrap();
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture("./hoge.jpg").unwrap();
    let black = sdl2::pixels::Color::RGB(0, 0, 0);
    let white = sdl2::pixels::Color::RGB(255, 255, 255);
    let mut events = ctx.event_pump().unwrap();

    let mut circlePosition = CirclePosition{
        x: 300, y:200, direction: Direction::east as i16, radius: 30,
        color: white, isOpeningMouth: true
    };

    let device = createDeviceMusic(&audio_subsystem);

    let fifty_millis = time::Duration::from_millis(50);
    let mut main_loop = || {
        thread::sleep(fifty_millis);
        circlePosition.moveMouth();

        for event in events.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                    process::exit(1);
                },
                Event::KeyDown { keycode: Some(Keycode::Left), ..} => {
                    circlePosition.x -= 10;
                    circlePosition.direction = Direction::west as i16;
                    device.resume();
                },
                Event::KeyDown { keycode: Some(Keycode::Right), ..} => {
                    circlePosition.x += 10;
                    circlePosition.direction = Direction::east as i16;
                },
                Event::KeyDown { keycode: Some(Keycode::Up), ..} => {
                    circlePosition.y -= 10;
                    circlePosition.direction = Direction::north as i16;
                },
                Event::KeyDown { keycode: Some(Keycode::Down), ..} => {
                    circlePosition.y += 10;
                    circlePosition.direction = Direction::south as i16;
                },
                _ => {}
            }
        }
        canvas.set_draw_color(black);
        canvas.clear();
        canvas.set_draw_color(white);
        circlePosition.movePosition(&canvas);
        canvas.present();
    };

    loop { main_loop(); }
}