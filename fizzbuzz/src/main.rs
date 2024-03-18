use std::io;

fn main() {
    println!("Enter a number");

    let mut number = String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("Please enter a number\n");

    let number: u32 = number.trim().parse().expect("Enter a valid integer\n");

    for i in 1..number + 1 {
        if i % 3 == 0 && i % 5 == 0 {
            println!("FizzBuzz!");
        } else if i % 3 == 0 {
            println!("Fizz");
        } else if i % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{i}");
        }
    }
}
