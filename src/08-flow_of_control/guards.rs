#[allow(dead_code)]
enum Temperature {
    Celsius(i32),
    Fahrenheit(i32),
}

fn main() {
    let temperature = Temperature::Fahrenheit(35);

    match temperature {
        Temperature::Celsius(a) if a > 30 => println!("{}C is above 30 Celsius", a),
        Temperature::Celsius(a) => println!("{}C is equal or below 30 Celsius", a),
        Temperature::Fahrenheit(a) if a > 86 => println!("{}F is above 86 Fahrenheit", a),
        Temperature::Fahrenheit(a) => println!("{}F is equal or below 86 Fahrenheit", a),
    }
}
