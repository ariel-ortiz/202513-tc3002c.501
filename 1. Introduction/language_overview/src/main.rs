fn main() {
    hello();
    let x = 34;
    println!("{}! = {}", x, fact(x));
}

fn hello() {
    println!("Hello, world!");
}

fn fact(n: u8) -> u128 {
    let mut result: u128 = 1;
    for i in 1..=n {
        result *= i as u128;
    }
    result
}
