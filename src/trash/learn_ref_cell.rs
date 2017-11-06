use std::cell::RefCell;

#[derive(Debug)]
struct Poyo {
    x: i16,
    y: u16
}

fn main() {

    use std::cell::RefCell;
    let c = RefCell::new(Poyo{x: 1, y: 2});
    c.borrow_mut().x = 3;

    let five = c.into_inner();
    println!("{:?}", five);
}