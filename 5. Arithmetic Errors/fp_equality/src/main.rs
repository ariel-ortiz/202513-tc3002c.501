use approx::relative_eq;

fn main() {
    let tenth: f64 = 0.1;
    let three_tenths: f64 =  tenth + tenth + tenth;
    println!("{}", three_tenths);
    println!("{}", three_tenths == 0.3);
    println!("{}", relative_eq!(three_tenths, 0.3));
    println!("{}", relative_eq!(0.999, 1.0, epsilon=0.000001));
    println!("{}", relative_eq!(0.999, 1.0, epsilon=0.01));
}
