use std::{fs, env, io, fmt, process};
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn extract_file_data(path: &Path) -> String {
    let mut file = match File::open(&path) {
        Err(why) => panic!(why),
        Ok(file) => file
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!(why),
        Ok(_) => {}
    };

    return s;
}

fn main() {
    let csv = extract_file_data(Path::new("test.csv"));
    let v: Vec<&str> = csv.split(",").collect();

    println!("{}", v[0]);
}