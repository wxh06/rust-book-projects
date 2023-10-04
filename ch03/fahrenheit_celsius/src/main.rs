use std::io;

fn fahrenheit_to_celsius(degrees: f64) -> f64 {
    (degrees - 32.0) / 1.8
}

fn celsius_to_fahrenheit(degrees: f64) -> f64 {
    degrees * 1.8 + 32.0
}

fn main() {
    let mut degrees = String::new();
    io::stdin().read_line(&mut degrees).unwrap();
    let degrees = degrees.trim().parse().unwrap();

    let mut scale = String::new();
    io::stdin().read_line(&mut scale).unwrap();

    print!("{} degree(s) ", degrees);
    if scale.trim().to_ascii_lowercase() == "fahrenheit" {
        let degrees = fahrenheit_to_celsius(degrees);
        println!("Fahrenheit = {} degree(s) Celsius", degrees);
    } else if scale.trim().to_ascii_lowercase() == "celsius" {
        let degrees = celsius_to_fahrenheit(degrees);
        println!("Celsius = {} degree(s) Fahrenheit", degrees);
    }
}
