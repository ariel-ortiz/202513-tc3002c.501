fn main() {
    hello();
    let x = 34;
    println!("{}! = {}", x, fact(x));

    let s1: i32 = sign(5);
    let s2: i32 = sign(-10);
    let s3: i32 = sign(0);

    println!("s1 = {}", s1);
    println!("s2 = {}", s2);
    println!("s3 = {}", s3);

    let n = -100;
    match sign(n) {
        -1 => println!("{} es negativo", n),
        1  => println!("{} es positivo", n),
        _  => println!("{} es cero", n) 
    }
}

fn hello() {
    println!("Hello, world!");
}

fn fact(n: u8) -> u128 {
    let mut result: u128 = 1;

    // for i in 1..=n {
    //     result *= i as u128;
    // }

    let mut i: u8 = 1;

    // while i <= n {
    //     result *= i as u128;
    //     i += 1;
    // }

    loop {
        if i > n {
            break;
        }
        result *= i as u128;
        i += 1;
    }

    result
}

fn sign(n: i32) -> i32 {
    if n < 0 {
        -1
    } else if n > 0 {
        1
    } else {
        0
    }
}
