fn main() {
    let v = vec![4, 8, 15, 16, 23, 42];
    // let v = Vec::<i32>::new();
    let x: Option<f64> = average(&v);

    // Primera opción: unwrap
    // "cobrarse a lo chino": quitar la envoltura suponiendo que no regresa None
    // Si x es None, el programa se apanica
    // let r = x.unwrap();
    // let r = x.expect("No se puede calcular el promedio de un vector vacío");

    // Segunda opción: match
    // let r = match x {
    //     Some(result) => result,
    //     None => 0.0
    // };

    // Tercera opción: if let
    // let r = if let Some(result) = x {
    //     result
    // } else {
    //     0.0
    // };

    // Cuarta opción: is_none
    let r = if x.is_none() {
        0.0
    } else {
        x.unwrap() // no puede fallar
    };

    println!("{:.2}", r);
}

fn average(data: &Vec<i32>) -> Option<f64> {
    let length: usize = data.len();
    if length == 0 {
        return None;
    }
    let sum: i32 = data.iter().sum();
    Some(sum as f64 / length as f64)
}
