use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
// use std::{fs, env, io, fmt, process};
// use std::error::Error;   
// use std::num::ParseIntError;

use std::io::Error as IoError;
use std::fmt;

pub type Result<T> = std::result::Result<T, CustomError>;
#[derive(Debug)]
pub enum CustomError {
    ParseString(String),
    ParseIoError(IoError),
    ParseI16(i16)
}

trait Error: fmt::Debug + fmt::Display {
    fn description(&self) -> &str;
    fn cause(&self) -> Option<&Error>;
}

impl From<IoError> for CustomError {
    fn from(err: IoError) -> CustomError {
        CustomError::ParseIoError(err)
    }
}

impl From<String> for CustomError {
    fn from(err: String) -> CustomError {
        CustomError::ParseString(err)
    }
}

impl From<i16> for CustomError {
    fn from(err: i16) -> CustomError {
        CustomError::ParseI16(err)
    }
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result<> {
        match *self {
            CustomError::ParseString(ref e) => e.fmt(f),
            CustomError::ParseIoError(ref e) => e.fmt(f),
            CustomError::ParseI16(ref e) => e.fmt(f)
        }
    }
}

fn extract_file_data(path: &Path) -> Result<String> {
    let mut file = File::open(&path)?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    return Ok(s);
}

fn is_below_ten(num: i16) -> Result<i16> {
    return match num {
        num if num < 10 => Err(CustomError::ParseI16(num)),
        _ => Ok(num)
    };
}

fn is_over_twenty(num: i16) -> Result<i16> {
    return match num {
        num if 20 < num => Err(CustomError::ParseI16(num)),
        _ => Ok(num)
    };
}

fn extract_data_from_line(num: i16) -> Result<i16> {
    let _ = is_below_ten(num)?;
    let _ = is_over_twenty(num)?;
    return Ok(num);
    // return is_below_ten(num)
    //             .map(|n| n)
    //             .map_err(|err| err);
}

fn main() {
    let csv = match extract_file_data(Path::new("test.csv")){
                Err(why) => panic!("{}", why),
                Ok(file) => file
            };

    let lines: Vec<&str> = csv.lines().collect();
    // let _vec: Vec<&str> = csv.split("\n").collect();

    let aaa: i16 = match extract_data_from_line(9) {
                        Err(why) => panic!("{} is less than 10.", why),
                        Ok(data) => data
                    };

    println!("{:?}", aaa);

    // for line in lines {
    //     let _vec: Vec<&str> = line.split(",").collect();
    //     println!("{:?}", _vec);
    // }
 }