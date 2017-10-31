macro_rules! expr { ($x:expr) => ($x) } // avoid for macro bug
macro_rules! tuple_index {
    ($tuple:expr, $idx:tt) => { expr!($tuple.$idx) }
}
const hoge:(i8, i16, i32) = (3, 5, 7);

trait Nyan {
    fn abeshi(&self);
}
impl Nyan for i8 {
    fn abeshi(&self) {
        println!("i8");
    }
}
impl Nyan for i16 {
    fn abeshi(&self) {
        println!("i16");
    }
}
impl Nyan for i32 {
    fn abeshi(&self) {
        println!("i32");
    }
}

// type2
// trait Hoge {
//     fn aaa() {}
// }
// impl Hoge for i16 {
//     fn aaa() {
//         println!("i16");
//     }
// }
// impl Hoge for str{
//     fn aaa() {
//         println!("")
//     }
// }

// struct Poyo<T> {
//     aaa: T
// }
// impl <T>Poyo<T> {
//     fn hoge(&self) -> &T {
//         return &self.aaa;
//     }
// }
// fn createPoyo<T>(aaa: T) -> Poyo<T> {
//     Poyo {
//         aaa: aaa
//     }
// }

// let abc = createPoyo("def");
// println!("{}", abc.aaa);
