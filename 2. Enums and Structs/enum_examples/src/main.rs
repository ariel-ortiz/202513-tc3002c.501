#[derive(Debug)]
enum CardinalPoints {
    North, South, East, West
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
}
