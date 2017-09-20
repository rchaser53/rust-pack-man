extern crate csv;
extern crate getopts;
extern crate rustc_serialize;

// use getopts::Options;
use std::path::Path;
use std::{fs, env, io, fmt, process};
use std::error::{Error};

use std::fs::File;
use std::io::Read;
// use std::io::prelude::*;

pub mod error_handling;
use error_handling::{double_first, print_double_first};

// impl fmt::Display for CliError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match *self {
//             CliError::Io(ref err) => err.fmt(f),
//             CliError::Csv(ref err) => err.fmt(f),
//             CliError::NotFound => write!(f, "No matching cities with a \
//                                              population were found."),
//         }
//     }
// }

// impl Error for CliError {
//     fn description(&self) -> &str {
//         match *self {
//             CliError::Io(ref err) => err.description(),
//             CliError::Csv(ref err) => err.description(),
//             CliError::NotFound => "not found",
//         }
//     }
// }

// impl From<io::Error> for CliError {
//     fn from(err: io::Error) -> CliError {
//         CliError::Io(err)
//     }
// }

// impl From<csv::Error> for CliError {
//     fn from(err: csv::Error) -> CliError {
//         CliError::Csv(err)
//     }
// }

// #[derive(Debug)]
// enum CliError {
//     Io(io::Error),
//     Csv(csv::Error),
//     NotFound,
// }

// #[derive(Debug, RustcDecodable)]
// struct Row {
//     country: String,
//     city: String,
//     accent_city: String,
//     region: String,

//     population: Option<u64>,
//     latitude: Option<f64>,
//     longitude: Option<f64>,
// }

// struct PopulationCount {
//     city: String,
//     country: String,
//     count: u64,
// }

// fn print_usage(program: &str, opts: Options) {
//     println!("{}", opts.usage(&format!("Usage: {} [options] <city>", program)));
// }

// fn search<P: AsRef<Path>>
//          (file_path: &Option<P>, city: &str)
//          -> Result<Vec<PopulationCount>, CliError> {
//     let mut found = vec![];
//     let input: Box<io::Read> = match *file_path {
//         None => Box::new(io::stdin()),
//         Some(ref file_path) => Box::new(try!(fs::File::open(file_path))),
//     };
//     let mut rdr = csv::Reader::from_reader(input);
//     for row in rdr.decode::<Row>() {
//         let row = try!(row);
//         match row.population {
//             None => { } // スキップする
//             Some(count) => if row.city == city {
//                 found.push(PopulationCount {
//                     city: row.city,
//                     country: row.country,
//                     count: count,
//                 });
//             },
//         }
//     }
//     if found.is_empty() {
//         Err(CliError::NotFound)
//     } else {
//         Ok(found)
//     }
// }

// fn main() {
//     let numbers = vec!["93", "18"];
//     let empty = vec![];
//     let strings = vec!["tofu", "93", "18"];

//     print_double_first(double_first(numbers));
//     print_double_first(double_first(empty));
//     print_double_first(double_first(strings));
// }

fn cat(path: &Path) -> Result<String, std::io::Error> {
    // let mut f = try!(File::open(path));
    // let mut f = File::open(path)?;
    let mut f = match File::open(path) {
        Err(why) => panic!("nya-n {}", why.description()),
        Ok(f) => f
    };

    let mut s = String::new();
    let _ = f.read_to_string(&mut s);
    return Ok(s);
}

fn main() {
    let ab = match cat(Path::new("test.csv")) {
        Err(why) => {
            println!("! {:?}", why.kind());
            panic!("nya");
        },
        Ok(s) => s
    };
    println!("{}", ab);

    process::exit(0);
}



    // let args: Vec<String> = env::args().collect();
    // let program = args[0].clone();

    // let mut opts = Options::new();
    // opts.optflag("h", "help", "Show this usage message.");

    // let matches = match opts.parse(&args[1..]) {
    //     Ok(m)  => { m }
    //     Err(e) => { panic!(e.to_string()) }
    // };
    // if matches.opt_present("h") {
    //     print_usage(&program, opts);
    //     return;
    // }

    // let data_file = args[1].clone();
    // let data_path = Path::new(&data_file);
    // let city = args[2].clone();
    // for pop in search(&data_path, &city) {
    //     println!("{}, {}: {:?}", pop.city, pop.country, pop.count);
    // }
    // let file_path = Path::new("test.csv");
    // file_path.display();

            // let mut contents = String::new();
            // let mut aaa;



    // println!("{}", hoge);
    // let args: Vec<String> = env::args().collect();
    // let program = args[0].clone();

    // let mut opts = Options::new();
    // opts.optopt("f", "file", "Choose an input file, instead of using STDIN.", "NAME");
    // opts.optflag("h", "help", "Show this usage message.");

    // let matches = match opts.parse(&args[1..]) {
    //     Ok(m)  => { m }
    //     Err(e) => { panic!(e.to_string()) }
    // };
    // if matches.opt_present("h") {
    //     print_usage(&program, opts);
    //     return;
    // }

    // let file = matches.opt_str("f");
    // let data_file = file.as_ref().map(Path::new);

    // let city = if !matches.free.is_empty() {
    //     matches.free[0].clone()
    // } else {
    //     print_usage(&program, opts);
    //     return;
    // };

    // for pop in search(&data_file, &city) {
        // println!("aa");
        // println!(pop);
        
        // println!("{}, {}: {:?}", pop.city, pop.country, pop.count);
    // }
// }