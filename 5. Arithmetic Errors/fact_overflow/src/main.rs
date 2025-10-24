fn main() {
    let x: u8 = 35;
    let r: Option<u128> = fact(x);
    println!("{}! = {:?}", x, r.unwrap_or(0));
    println!("{}! = {:?}", x, fact(34).unwrap_or(0));
}

fn fact(n: u8) -> Option<u128> {
    let mut result: u128 = 1;
    for i in 2..=n {
        result = match result.checked_mul(i as u128) {
            Some(x) => x,
            None => return None
        }
    }
    Some(result)
}
