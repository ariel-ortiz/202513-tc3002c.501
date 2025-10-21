fn main() {
    demo_checked();
}

fn demo_checked() {
    let x: u8 = 10;
    let y: u8 = 250;
    let z: Option<u8> = x.checked_add(y);
    match z {
        Some(valor) => println!("{}", valor),
        None => println!("No hay resultado")
    }
}
