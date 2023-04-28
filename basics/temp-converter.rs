use std::io;

fn celcius_to_kelvin(degree_in_celcius: f64) -> f64 {
    degree_in_celcius + 273.15
}

fn celcius_to_fahrenheit(degree_in_celcius: f64) -> f64 {
    (degree_in_celcius * 9.0 / 5.0) + 32.0
}

fn main() {
    let mut temp = String::new();

    io::stdin()
        .read_line(&mut temp)
        .expect("Error while reading user input!");

    let ftemp: f64 = temp.trim().parse().unwrap();

    println!(
        "{} degree celcius in kelvin is {}k",
        ftemp,
        celcius_to_kelvin(ftemp)
    );
    println!(
        "{} degree celcius in degree farhenheit is {}",
        ftemp,
        celcius_to_fahrenheit(ftemp)
    );
}
