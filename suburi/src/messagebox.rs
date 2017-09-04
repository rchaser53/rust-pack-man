extern crate sdl2;
use sdl2::messagebox;

pub fn showMessage(title: &str, message: &str) {
  messagebox::show_simple_message_box(
        messagebox::MESSAGEBOX_ERROR,
        title,
        message,
        None
    );
}