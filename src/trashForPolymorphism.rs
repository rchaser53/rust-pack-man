macro_rules! expr { ($x:expr) => ($x) } // avoid for macro bug
macro_rules! tuple_index {
    ($tuple:expr, $idx:tt) => { expr!($tuple.$idx) }
}
const hoge:(i8, i16, i32) = (3, 5, 7);

trait Nyan {
    fn abeshi(&self);
}
impl Nyan for i8 {
    fn abeshi(&self) -> () {
        println!("i8");
    }
}
impl Nyan for i16 {
    fn abeshi(&self) -> () {
        println!("i16");
    }
}
impl Nyan for i32 {
    fn abeshi(&self) -> () {
        println!("i32");
    }
}