use std::num::ParseIntError;

fn main() {
    let s: String = "42".to_string();
    let r: Result<i32, ParseIntError> = s.parse();

    let value = match r {
        Ok(x) => x,
        Err(_) => 0
    };

    println!("{}", value);
}
