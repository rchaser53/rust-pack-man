extern crate sdl2;

use std::env;
use std::path::Path;
use sdl2::mixer::{DEFAULT_CHANNELS, INIT_MP3, INIT_FLAC, INIT_MOD, INIT_FLUIDSYNTH, INIT_MODPLUG,
                  INIT_OGG, AUDIO_S16LSB};

fn setup_sdl2_mixier() -> () {
    let frequency = 44100;
    let format = AUDIO_S16LSB;          // signed 16 bit samples, in little-endian byte order
    let channels = DEFAULT_CHANNELS;    // Stereo
    let chunk_size = 1024;

    sdl2::mixer::open_audio(frequency, format, channels, chunk_size).unwrap();

    // Number of mixing channels available for sound effect `Chunk`s to play
    // simultaneously.
    sdl2::mixer::allocate_channels(4);
}

pub fn play_bgm(ctx: &sdl2::Sdl) -> sdl2::mixer::Music {
    let mut timer = ctx.timer().unwrap();
    let _mixer_context = sdl2::mixer::init(INIT_MP3 | INIT_FLAC | INIT_MOD | INIT_FLUIDSYNTH |
                                           INIT_MODPLUG | INIT_OGG).unwrap();
    setup_sdl2_mixier();
    return sdl2::mixer::Music::from_file(Path::new("nyan.mp3")).unwrap();
}