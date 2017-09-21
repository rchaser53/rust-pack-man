use std::{fs, env, io, fmt, process};
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;


fn extract_file_data(path: &Path) -> Result<String, std::io::Error> {
    let mut file = File::open(&path)?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    return Ok(s);
}

fn main() {
    let csv = match extract_file_data(Path::new("test.csv")){
                Err(why) => panic!(why),
                Ok(file) => file
            };

    let _vec: Vec<&str> = csv.lines().collect();
    // let _vec: Vec<&str> = csv.split("\n").collect();

    for v in _vec {
        println!("{} ___", v);
    }
 }