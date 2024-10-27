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

        let input: f64 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        };
        return input;
    }
}

fn get_operation() -> String {
    loop {
        println!("Enter the operation to be performed: Add, Subtract, Multiply, Divide");
        let mut operation = String::new();

        io::stdin().read_line(&mut operation).unwrap();

        let operation = match operation.trim() {
            op @ ("Add" | "Subtract" | "Multiply" | "Divide") => op,
            _ => {
                println!("invalid operation");
                continue;
            }
        };
        return operation.to_string();
    }
}

fn calculate(operation: Operation) -> f64 {
    match operation {
        Add(num1, num2) => num1 + num2,
        Subtract(num1, num2) => num1 - num2,
        Multiply(num1, num2) => num1 * num2,
        Divide(num1, num2) => num1 / num2,
    }
}

fn main() {
    println!("Enter your first number");
    let first_num = get_number();

    let operation = get_operation();

    println!("Enter your second number");
    let second_num = get_number();

    let operation = match operation.as_str() {
        "Add" => Add(first_num, second_num),
        "Subtract" => Subtract(first_num, second_num),
        "Multiply" => Multiply(first_num, second_num),
        "Divide" => Divide(first_num, second_num),
        _ => {
            panic!("Invalid operation");
        }
    };

    let result = calculate(operation);

    println!("{}", result);
}
