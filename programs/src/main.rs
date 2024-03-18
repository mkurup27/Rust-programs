use std::io;

fn main() {
    println!("Enter your temperature in Celsius:");

    let mut celsius = String::new();

    io::stdin()
        .read_line(&mut celsius)
        .expect("Please enter the temperature:");

    let celsius: f32 = celsius.trim().parse().expect("Please type a number");

    let fahrenheit: f32 = (celsius * (9.0 / 5.0)) + 32.0;

    println!("The temperature in Fahrenheit is {fahrenheit}");
}
