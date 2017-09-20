use std::path::Path;
use std::{fs, env, io, fmt, process};
use std::error::{Error};
use fmt::Binary;

use std::fs::File;
use std::io::Read;

#[derive(Debug)]
struct MinMax(i64, i64);

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "({}, {})", self.0, self.1)
    }
}

// #[derive(Debug)] - this means 'impl fmt::Debug for Point2
struct Point2 {
    x: f64,
    y: f64,
}

impl fmt::Display for Point2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}i", self.x, self.y)
    }
}

impl fmt::Debug for Point2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ real: {}, imag: {} }}", self.x, self.y)
    }
}

fn main() {
    let minmax = MinMax(0, 14);

    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range =   MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!("The big range is {big} and the small is {small}",
             small = small_range,
             big = big_range);

    let point = Point2 { x: 3.3, y: 7.2 };

    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);
}


// fn cat(path: &Path) -> Result<String, std::io::Error> {
//     // let mut f = try!(File::open(path));
//     // let mut f = File::open(path)?;
//     let mut f = match File::open(path) {
//         Err(why) => panic!("nya-n   {}", why.description()),
//         Ok(f) => f
//     };

//     let mut s = String::new();
//     let _ = f.read_to_string(&mut s);
//     return Ok(s);
// }

// fn main() {
//     let ab = match cat(Path::new("test.csv")) {
//         Err(why) => {
//             println!("! {:?}", why.kind());
//             panic!("nya");
//         },
//         Ok(s) => s
//     };
//     println!("{}", ab);

//     process::exit(0);
// }