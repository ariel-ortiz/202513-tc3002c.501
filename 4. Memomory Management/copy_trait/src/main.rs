#[derive(Debug, Copy, Clone)]
struct Fraction {
    numerator: i32,
    denominator: i32
}

fn main() {
    let v = vec![4, 10, 9];
    println!("{:?}", v);
    let w = v;  // move
    println!("{:?}", w);
    // println!("{:?}", v);

    let x: i32 = 7;
    println!("{}", x);
    let y = x;  // copy
    println!("{}", x);
    println!("{}", y);

    let f1 = Fraction {numerator: 1, denominator: 2};
    println!("f1 = {:?}", f1);
    let mut f2 = f1; // copy
    f2.denominator = 3;
    println!("f1 = {:?}", f1);
    println!("f2 = {:?}", f2);
    let mut f3 = f2.clone(); // copy
    f3.numerator = 2;
    println!("f3 = {:?}", f3);
    println!("f1 = {:?}", f2);
}
