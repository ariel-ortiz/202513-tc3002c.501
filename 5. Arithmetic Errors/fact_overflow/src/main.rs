fn main() {
    let x = 35;
    let r = fact(x);
    println!("{}! = {}", x, r);
}

fn fact(n: u8) -> u128 {
    let mut result: u128 = 1;
    for i in 2..=n {
        result *= i as u128;
    }
    result
}
