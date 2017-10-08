use sdl2::video::WindowBuildError;
use sdl2::messagebox::{show_simple_message_box, MESSAGEBOX_ERROR};

use std::{fmt, process, result};

#[derive(Debug)]
pub enum GameOverError {
    OtherError(&'static str)
}
impl fmt::Display for GameOverError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GameOverError::OtherError(ref e) => write!(f, "{}", e.to_string()),
        }
    }
}
impl Error for GameOverError {
    fn description(&self) -> &str {
        match *self {
            GameOverError::OtherError(error_message) => error_message,
        }
    }
    fn cause(&self) -> Option<&Error> { None }
}

pub type Result<T> = result::Result<T, CustomError>;
trait Error: fmt::Debug + fmt::Display {
    fn description(&self) -> &str;
    fn cause(&self) -> Option<&Error>;
}

#[derive(Debug)]
pub enum CustomError {
    ParseWindowBuildError(WindowBuildError),
    ParseString(String),
    ParseGameOverError(GameOverError)
}

impl From<GameOverError> for CustomError {
    fn from(err: GameOverError) -> CustomError {
        CustomError::ParseGameOverError(err)
    }
}

impl From<WindowBuildError> for CustomError {
    fn from(err: WindowBuildError) -> CustomError {
        CustomError::ParseWindowBuildError(err)
    }
}

impl From<String> for CustomError {
    fn from(err: String) -> CustomError {
        CustomError::ParseString(err)
    }
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result<> {
        match *self {
            CustomError::ParseWindowBuildError(_) => write!(f, "nya-n"),
            CustomError::ParseGameOverError(ref e) => {
                show_message("titler", e.description());
                process::exit(1);
            },
            CustomError::ParseString(ref e) => panic!("{}", e),
        }
    }
}

pub fn show_message(title: &str, message: &str) -> () {
  let _ = show_simple_message_box(
              MESSAGEBOX_ERROR, title,
              message, None
          );
}