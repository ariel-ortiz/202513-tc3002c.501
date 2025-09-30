#[derive(Debug)]
enum CardinalPoints {
    North, South, East, West
}

#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
    Black,
    White,
    RGB(u8, u8, u8)
}

fn main() {
    let point1 = CardinalPoints::North;
    let mut point2: CardinalPoints = CardinalPoints::East;

    println!("point2 = {:?}", point2);

    point2 = CardinalPoints::South;
    println!("point2 = {:?}", point2);

    point2 = CardinalPoints::West;

    println!("point1 = {:?}", point1);
    println!("point2 = {:?}", point2);

    match point2 {
        CardinalPoints::North => println!("Norte"),
        CardinalPoints::South => println!("Sur"),
        _ => println!("Otra")
    }

    let color1 = Color::Red;
    let color2 = Color::White;
    let color3 = Color::RGB(0, 255, 255);
    let colores = vec![Color::Green, Color::Blue, Color::Black];

    println!("{:?} {:?} {:?} {:?}", color1, color2, color3, colores);

    match color3 {
        Color::Red => println!("Soy rojo"),
        Color::Green => println!("Soy verde"),
        Color::Blue => println!("Soy azul"),
        Color::Black => println!("Soy negro"),
        Color::White => println!("Soy blanco"),
        Color::RGB(r, g, b) => println!("R={}, G={}, B={}", r, g, b)
    }
}
