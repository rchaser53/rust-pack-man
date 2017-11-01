trait SayHello : std::fmt::Display {
    fn say_hello(&self) {
        println!("Hello. This is {}.", self);
    }
}
impl SayHello for i32 {}
impl SayHello for char {
    fn say_hello(&self) {
        println!("pyaaaa- {}.", self);
    }
}

struct Gyan {
    aaa: Box<SayHello>
}

fn main() {
    let hoge = Gyan{aaa: Box::new('a') as Box<SayHello>};
    hoge.aaa.say_hello();
}
