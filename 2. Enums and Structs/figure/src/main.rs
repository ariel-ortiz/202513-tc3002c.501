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
        format!("Square (side = {} m)", self.side)
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

#[derive(Debug)]
struct Triangle {
    base: f64,
    height: f64
}


impl Triangle {
    fn new(base: f64, height: f64) -> Self {
        Self { base, height }
    }
}

impl Figure for Triangle {

    fn area(&self) -> f64 {
        self.base * self.height / 2.0
    }

    fn perimeter(&self) -> f64 {
        self.base * 3.0
    }

    fn description(&self) -> String {
        format!("Triangle (base = {} m, height = {} m)", self.base, self.height)
    }
}

fn report<T: Figure>(figure: &T) {
    println!("FIGURE: {}", figure.description());
    println!("------------------------------------");
    println!("Área = {:.2} m^2", figure.area());
    println!("Perímetro = {:.2} m", figure.perimeter());
    println!();
}

impl Figure for i32 {
    
    fn area(&self) -> f64 {
        0.0
    }

    fn perimeter(&self) -> f64 {
        0.0
    }

    fn description(&self) -> String {
        format!("i32 = {}", self)
    }
}

fn main() {
    let s = Square::new(3.0);
    report(&s);

    let c = Circle::new(10.0);
    report(&c);

    let t = Triangle::new(4.5, 2.0);
    report(&t);

    let x: i32 = 7;
    report(&x);
}
