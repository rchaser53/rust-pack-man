use std::{fs, env, io, fmt, process};
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::num::ParseIntError;


fn extract_file_data(path: &Path) -> Result<String, std::io::Error> {
    let mut file = File::open(&path)?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    return Ok(s);
}

fn extractDataFromLine(a: i16) -> Result<i16, String> {
    return match a {
        a if a < 10 => Err(a.to_string()),
        _ => Ok(a)
    };
}

fn main() {
    let csv = match extract_file_data(Path::new("test.csv")){
                Err(why) => panic!("{}", why),
                Ok(file) => file
            };

    let lines: Vec<&str> = csv.lines().collect();
    // let _vec: Vec<&str> = csv.split("\n").collect();

    // let aaa: i16 = match extractDataFromLine(9) {
    let aaa: i16 = match extractDataFromLine(10) {
                        Err(why) => panic!("{} is less than 10.", why),
                        Ok(data) => data
                    };

    println!("{:?}", aaa);

    // for line in lines {
    //     let _vec: Vec<&str> = line.split(",").collect();
    //     println!("{:?}", _vec);
    // }
 }