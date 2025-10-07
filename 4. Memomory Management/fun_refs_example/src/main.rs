fn main() {
    let v: Vec<u8> = vec![4, 8, 15, 16, 23, 42];
    let u = v; // move
    let w: Vec<u8>;
    w = u; // move
    println!("{:?}", w);

    let x = fun1(w); // 2 move
    println!("{:?}", x);

    fun2(&x);

    let mut y = x.clone();
    fun3(&mut y);
    println!("{:?}", y);

    let mut z: i32 = 10;
    println!("z = {}", z);
    fun4(&mut z);
    println!("z = {}", z);
}

fn fun1(v: Vec<u8>) -> Vec<u8> {
    println!("{:?}", v);
    v // move
}

fn fun2(v: &Vec<u8>) {
    println!("{:?}", (*v)[0]);
}

fn fun3(v: &mut Vec<u8>) {
    (*v)[0] = 0;
}

fn fun4(x: &mut i32) {
    (*x) += 1;
}
