use std;
// use std::error::{Error};
use std::num::ParseIntError;
use std::fmt;

pub type Result<T> = std::result::Result<T, DoubleError>;

trait Error: fmt::Debug + fmt::Display {
    fn description(&self) -> &str;
    fn cause(&self) -> Option<&Error>;
}

#[derive(Debug)]
pub enum DoubleError {
    EmptyVec,
    Parse(ParseIntError),
}

impl From<ParseIntError> for DoubleError {
    fn from(err: ParseIntError) -> DoubleError {
        DoubleError::Parse(err)
    }
}

impl fmt::Display for DoubleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result<> {
        match *self {
            DoubleError::EmptyVec =>
                write!(f, "please use a vector with at least one element"),
            DoubleError::Parse(ref e) => e.fmt(f),
        }
    }
}

// 前と同じ構成だが、全`Result`と`Option`をチェインしていく代わりに、
// いきなり中の値を取り出すことに`try!`している。
pub fn double_first(vec: Vec<&str>) -> Result<i32> {
    // Still convert to `Result` by stating how to convert `None`.
    let first = try!(vec.first().ok_or(DoubleError::EmptyVec));
    let parsed = try!(first.parse::<i32>());

    Ok(2 * parsed)
}

pub fn print_double_first(result: Result<i32>) -> () {
    let _ = match result {
        Ok(n)  => println!("The first doubled is {}", n),
        Err(e) => println!("Error: {}", e),
    };
}

// fn main() {
//     let numbers = vec!["93", "18"];
//     let empty = vec![];
//     let strings = vec!["tofu", "93", "18"];

//     print_double_first(double_first(numbers));
//     print_double_first(double_first(empty));
//     print_double_first(double_first(strings));
// }