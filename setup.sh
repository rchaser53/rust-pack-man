curl https://sh.rustup.rs -sSf | sh
brew install sdl2 sdl2_gfx sdl2_image sdl2_ttf
brew install sdl2_mixer --with-flac --with-fluid-synth --with-libmikmod --with-libmodplug --with-libvorbis --with-smpeg2
cargo install cargo-edit