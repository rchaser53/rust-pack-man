use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
// use std::{fs, env, io, fmt, process};
// use std::error::Error;
// use std::num::ParseIntError;

fn extract_file_data(path: &Path) -> Result<String, std::io::Error> {
    let mut file = File::open(&path)?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    return Ok(s);
}

fn is_below_ten(num: i16) -> Result<i16, String> {
    return match num {
        num if num < 10 => Err(num.to_string()),
        _ => Ok(num)
    };
}

fn is_over_twenty(num: i16) -> Result<i16, String> {
    return match num {
        num if 20 < num => Err(num.to_string()),
        _ => Ok(num)
    };
}

fn extract_data_from_line(num: i16) -> Result<i16, String> {
    return is_below_ten(num).and_then(is_over_twenty);
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

    let aaa: i16 = match extract_data_from_line(10) {
                        Err(why) => panic!("{} is less than 10.", why),
                        Ok(data) => data
                    };

    println!("{:?}", aaa);

    // for line in lines {
    //     let _vec: Vec<&str> = line.split(",").collect();
    //     println!("{:?}", _vec);
    // }
 }