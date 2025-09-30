use std::f64::consts::PI;


trait Figure {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
    fn description(&self) -> String {
        "unknown".to_string()
    }
}

#[derive(Debug)]
struct Square {
    side: f64
}

impl Square {
    fn new(side: f64) -> Self {
        Self { side }
    }
}

impl Figure for Square {

    fn area(&self) -> f64 {
        self.side * self.side
    }

    fn perimeter(&self) -> f64 {
        self.side * 4.0
    }

    fn description(&self) -> String {
        "Un cuadrado".to_string()
    }
}

#[derive(Debug)]
struct Circle {
    radius: f64
}

impl Circle {
    fn new(radius: f64) -> Self {
        Self { radius }
    }
}

impl Figure for Circle {

    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }

    fn perimeter(&self) -> f64 {
        PI * self.radius * 2.0
    }
}

fn main() {
    let s = Square::new(3.0);
    println!("{:?}", s);
    println!("Área = {}", s.area());
    println!("Perímetro = {}", s.perimeter());
    println!("Descripción = {}", s.description());

    let c = Circle::new(10.0);
    println!("{:?}", c);
    println!("Área = {}", c.area());
    println!("Perímetro = {}", c.perimeter());
    println!("Descripción = {}", c.description());
}
