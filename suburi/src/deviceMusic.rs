extern crate sdl2;

use std;
use std::borrow::Cow;
use std::path::{PathBuf, Path};

use sdl2::audio::{AudioCallback, AudioSpecDesired, AudioSpecWAV, AudioCVT};

pub struct Sound {
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
        self.pos = 0;
    }
}

pub fn createDeviceMusic(ctx: &sdl2::Sdl, path: &'static Path) -> sdl2::audio::AudioDevice<Sound> {
    let audio_subsystem = ctx.audio().unwrap();

    let wav_file: Cow<'static, Path> = match std::env::args().nth(1) {
        None => Cow::from(path),
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
            pos: 0
        }
    }).unwrap();
}