struct Nyan {
}

impl Nyan {
    fn aon() {
        println!("aon")
    }

    fn gyan(self) {
        println!("gyan")
    }
}

fn main() {
    println!("gy-n");
    Nyan::aon();
    let nyan: Nyan = Nyan{};
    nyan.gyan();
}