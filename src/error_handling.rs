use sdl2::video::WindowBuildError;

use std::{fmt, result};

pub type Result<T> = result::Result<T, CustomError>;
trait Error: fmt::Debug + fmt::Display {
    fn description(&self) -> &str;
    fn cause(&self) -> Option<&Error>;
}

#[derive(Debug)]
pub enum CustomError {
    ParseWindowBuildError(WindowBuildError),
    ParseString(String)
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
            CustomError::ParseString(ref e) => panic!("{}", e),
        }
    }
}