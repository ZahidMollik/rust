use std::io;

enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

fn calculate(op: Operation) -> f64 {
    match op {
        Operation::Add(num1, num2) => num1 + num2,
        Operation::Subtract(num1, num2) => num1 - num2,
        Operation::Multiply(num1, num2) => num1 * num2,
        Operation::Divide(num1, num2) => num1 / num2,
    }
}

fn main() {
    println!("Enter first number:");
    let mut num1 = String::new();
    io::stdin().read_line(&mut num1).expect("fail to read");

    println!("Enter the Operation(Add,Subtract,Multiply,Divide):");
    let mut operation = String::new();
    io::stdin().read_line(&mut operation).expect("fail to read");

    println!("Enter 2nd number:");
    let mut num2 = String::new();
    io::stdin().read_line(&mut num2).expect("fail to read");

    let num1 = num1.trim().parse().expect("Enter a valid number");
    let num2 = num2.trim().parse().expect("Enter a valid number");
    let operation = operation.trim();

    let op = match operation {
        "Add" => Operation::Add(num1, num2),
        "Subtract" => Operation::Subtract(num1, num2),
        "Multiply" => Operation::Multiply(num1, num2),
        "Divide" => Operation::Divide(num1, num2),
        _ => panic!("invalid operation"),
    };
    let result = calculate(op);
    println!("{}", result);
}
