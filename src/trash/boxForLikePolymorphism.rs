trait SayHello {
    fn say_hello(&self) {
        println!("Hello. This is");
    }
}
// impl SayHello for i32 {}
// impl SayHello for char {
//     fn say_hello(&self) {
//         println!("pyaaaa- {}.", self);
//     }
// }

struct Struct1 {
    bbb: i16
}
impl SayHello for Struct1 {}
struct Struct2 {
    bbb: i16
}
impl SayHello for Struct2 {
    fn say_hello(&self) {
        println!("pyaaaa-");
    }
}

struct Gyan {
    aaa: Box<SayHello>
}

fn main() {
    let hoge = Gyan{
                    aaa: Box::new(
                        Struct1{
                            bbb: 11
                        }
                    )
                };
    hoge.aaa.say_hello();
}