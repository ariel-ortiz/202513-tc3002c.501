fn main() {
    let x: u8 = 10;
    let y: i32 = x as i32;
    println!("{} {}", x, y);

    let x: i32 = 300;
    let y: u8 = x as u8;
    println!("{} {}", x, y);

    let x: f32 = 123.5;
    let y: u8 = x as u8;
    println!("{} {}", x, y);

    let x: f32 = 999.9;
    let y: u8 = x as u8;
    println!("{} {}", x, y);

    let x: f32 = -999.9;
    let y: i8 = x as i8;
    println!("{} {}", x, y);

    let x: i32 = 1_234_567_890;
    let y: f32 = x as f32;
    println!("{} {:.2}", x, y);

    let x: i32 = 1_234_567_890;
    let y: f64 = x as f64;
    println!("{} {:.2}", x, y);

    let x: Option<i32> = f64_to_i32(123.4);
    let y: Option<i32> = f64_to_i32(123E123);
    println!("{:?} {:?}", x, y);

}

fn f64_to_i32(n: f64) -> Option<i32> {
    if !n.is_finite() { // n is NaN, inf, -inf
        return None;
    }
    let truncated: f64 = n.trunc();
    if truncated >= i32::MIN as f64 && truncated <= i32::MAX as f64 {
        return Some(truncated as i32);
    }
    None
}
