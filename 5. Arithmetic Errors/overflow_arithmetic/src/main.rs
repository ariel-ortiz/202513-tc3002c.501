use std::u8;

fn main() {
    demo_checked();
    demo_overflowing();
    demo_saturating();
    demo_wrapping();
}

fn demo_checked() {
    let x: u8 = 10;
    let y: u8 = 250;
    let z: Option<u8> = x.checked_add(y);
    match z {
        Some(valor) => println!("{}", valor),
        None => println!("No hay resultado")
    }
    let x: u8 = 1;
    let y: u8 = 250;
    let z: u8 = x.checked_add(y).unwrap_or(u8::MAX);
    println!("{}", z);
    let x: u8 = 10;
    let y: u8 = 250;
    let z: u8 = x.checked_add(y).unwrap_or(u8::MAX);
    println!("{}", z);
}

fn demo_overflowing() {
    let x: u8 = 10;
    let y: u8 = 250;
    let z: (u8, bool) = x.overflowing_add(y);
    println!("{:?}", z);
    let x: u8 = 1;
    let y: u8 = 250;
    let z: (u8, bool) = x.overflowing_add(y);
    println!("{:?}", z);
}

fn demo_saturating() {
    let x: u8 = 10;
    let y: u8 = 250;
    let z: u8 = x.saturating_add(y);
    println!("{}", z);

    let x: u8 = 10;
    let y: u8 = 20;
    let z: u8 = x.saturating_sub(y);
    println!("{}", z);

    let x: u8 = 10;
    let y: u8 = 7;
    let z: u8 = x.saturating_sub(y);
    println!("{}", z);
}

fn demo_wrapping() {
    let x: u8 = 10;
    let y: u8 = 250;
    let z: u8 = x.wrapping_add(y);
    println!("{}", z);

    let x: u8 = 1;
    let y: u8 = 250;
    let z: u8 = x.wrapping_add(y);
    println!("{}", z);

}
