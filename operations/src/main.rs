use std::io;

fn main() {
    println!("Enter the first number:");

    let mut x = String::new();
    io::stdin().read_line(&mut x).expect("Enter a number!");
    let x = x.trim().parse().expect("Enter a number");

    println!("Enter the second number:");

    let mut y = String::new();
    io::stdin().read_line(&mut y).expect("Enter a number!");
    let y = y.trim().parse().expect("Enter a number");

    println!("Please choose the operation that you want to perform: \n 1. Addition \n 2. Subtraction \n 3. Multiplication \n 4. Division");

    let mut z = String::new();
    io::stdin().read_line(&mut z).expect("Enter a number!");
    let z: i32 = z.trim().parse().expect("Enter a number");

    if z < 1 || z > 4 {
        println!("Invalid option. Please select the correct option.")
    } else if z == 1 {
        let answer = addition(x, y);
        println!("The result is: {}", answer);
    } else if z == 2 {
        let answer = subtraction(x, y);
        println!("The result is: {}", answer);
    } else if z == 3 {
        let answer = multiplication(x, y);
        println!("The result is: {}", answer);
    } else {
        let answer = division(x, y);
        println!("The result is: {}", answer);
    }
}

fn addition(x: i32, y: i32) -> i32 {
    let num = x + y;
    num
}

fn subtraction(x: i32, y: i32) -> i32 {
    let num = x - y;
    num
}

fn multiplication(x: i32, y: i32) -> i32 {
    let num = x * y;
    num
}

fn division(x: i32, y: i32) -> i32 {
    let num = x / y;
    num
}
