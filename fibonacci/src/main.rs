use std::io;

fn main() {
    println!("Enter a number: ");

    let mut number = String::new();

    io::stdin().read_line(&mut number).expect("Enter a number!");

    let number = number.trim().parse().expect("Please type a number");

    let mut a = 0;

    let mut b = 1;

    let mut c = 0;
    if number < 0 {
        println!("Incorrect value! Please enter a positive number.");
    } else if number == 0 {
        println!("{a}");
    } else if number == 1 {
        println!("{b}");
    } else {
        let mut i = 2;
        while i <= number {
            c = a + b;
            a = b;
            b = c;
            i += 1;
        }
        println!("{c}");
    }
}
