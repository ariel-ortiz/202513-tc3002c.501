fn main() {
    let v1: Vec<i32> = vec![4, 8, 15, 16, 23, 42];
    let v2: Vec<i32> = v1.iter().map(|x: &i32| (*x) * 2).collect();
    let v3: Vec<&i32> = v1.iter().filter(|x: &&i32| (**x) % 2 == 0).collect();
    let sum: i32 = v1.iter().copied().reduce(|accum: i32, x: i32| accum + x).unwrap_or(0);

    println!("v1 = {:?}", v1);
    println!("v2 = {:?}", v2);
    println!("v3 = {:?}", v3);
    println!("sum = {:?}", sum);

    let f: fn(i32) -> f64 = |x: i32| (x as f64).sqrt();
    println!("f(2) = {}", f(2));
}
