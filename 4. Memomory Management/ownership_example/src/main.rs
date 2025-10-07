fn main() {
    let v: Vec<i32> = vec![4, 8, 15, 16, 23, 42];
    let mut u = v;  // move (u es el nuevo dueño del vector)
    let mut w = u.clone();
    w.push(108);
    u.push(0);
    println!("u = {:?}, w = {:?}", u, w);

    let mut a: Vec<i32> = vec![1, 2, 3];
    {
        let b = &a;
        let c = &a;
        let d = &a;
        println!("{:?}", b);
        println!("{:?}", c);
        println!("{:?}", d);
    }
    let e = &mut a;
    e.push(4);
    println!("{:?}", e);
}
