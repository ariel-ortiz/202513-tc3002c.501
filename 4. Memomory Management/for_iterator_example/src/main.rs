fn main() {
    let v: Vec<String> = vec![
        "Hugo".to_string(),
        "Paco".to_string(),
        "Luis".to_string()
    ];
    for_ver1(v.clone());
    for_ver2(v.clone());
    for_ver3(v.clone());
}

fn for_ver1(v: Vec<String>) {
    // For con "ownership"
    // Ocurre un move del vector y de sus elementos
    for elem in v { // for elem in v.into_iter()
        println!("{}", elem);
    }
    // println!("{:?}", v);
}

fn for_ver2(v: Vec<String>) {
    // For con "borrowing" de lectura
    for elem in &v { // for in v.iter()
        println!("{}", *elem);
    }
    println!("{:?}", v);
}

fn for_ver3(v: Vec<String>) {
    let mut w = v.clone();
    // For con "borrowing" de escritura
    for elem in &mut w {  // for in w.iter_mut()
        elem.push('*');
        println!("{}", elem);
    }
    w.push("Donald".to_string());
    println!("{:?}", w);
}
