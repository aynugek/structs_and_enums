use std::io;

enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

use Operation::*;

fn get_number() -> f64 {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim().parse::<f64>() {
            Ok(num) => return num,
            Err(_) => println!("Only numbers are allowed!"),
        }
    }
}

fn get_operation() -> Operation {
    loop {
        println!(
            "Enter the operation to perform (Add | + | Subtract | - | Multiply | * | Divide | /):"
        );
        let mut operation = String::new();
        io::stdin().read_line(&mut operation).unwrap();

        match operation.to_lowercase().trim() {
            "add" | "+" => return Add(0.0, 0.0),
            "subtract" | "-" => return Subtract(0.0, 0.0),
            "multiply" | "*" => return Multiply(0.0, 0.0),
            "divide" | "/" => return Divide(0.0, 0.0),
            _ => (),
        }
    }
}

fn calculate(operation: Operation) -> f64 {
    match operation {
        Add(num1, num2) => num1 + num2,
        Subtract(num1, num2) => num1 - num2,
        Multiply(num1, num2) => num1 * num2,
        Divide(num1, num2) => {
            if num2 != 0.0 {
                num1 / num2
            } else {
                f64::NAN
            }
        }
    }
}

fn main() {
    println!("Enter your first number:");
    let first_num = get_number();

    let mut operation = get_operation();

    println!("Enter your second number:");
    let second_num = get_number();

    operation = match operation {
        Add(..) => Add(first_num, second_num),
        Subtract(..) => Subtract(first_num, second_num),
        Multiply(..) => Multiply(first_num, second_num),
        Divide(..) => Divide(first_num, second_num),
    };

    let result = calculate(operation);

    println!("{}", result);
}
