fn main() {
    let zero:       f64 =  0.0;
    let minus_zero: f64 = -0.0;
    let one:        f64 =  1.0;
    println!("{}", zero == minus_zero);
    println!("{}", one / zero);
    println!("{}", one / minus_zero);
    let nan: f64 = zero / zero;
    println!("{}", nan);
    println!("{}", nan == nan);

    // let zero: i32 = 0;
    // let one: i32 = 1;
    // println!("{}", one / zero);
    println!("{}", (one / zero) as i8);
    println!("{}", (one / minus_zero) as i8);
    println!("{}", nan as i8);
}
