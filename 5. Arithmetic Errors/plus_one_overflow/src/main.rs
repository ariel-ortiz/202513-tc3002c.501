fn main() {
    // let x: u8 = 0b1111_1111;
    let y: u8 = 0b0000_0001;
    // let z: u8 = sum(x, y);
    let w: u8 = sub(0, y);
    // println!("{}", z);
    println!("{}", w);
    println!("{}", sum(250, 50));
    println!("{}", mul(50, 100));
}

fn sum(x: u8, y: u8) -> u8 {
    x + y
}

fn sub(x: u8, y: u8) -> u8 {
    x - y
}

fn mul(x: u8, y: u8) -> u8 {
    x * y
}
