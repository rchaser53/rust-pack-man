pub struct Nyan {
    x: i32,
    y: i16
}

fn aaa(temp: &Nyan) {
    bbb(*temp);
}

fn bbb(temp: Nyan) {
    println!("{}", temp.x);
}