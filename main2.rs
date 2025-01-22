use std::io;

fn main(){
    // Prompt for the first number
    println!("Enter the first number:");
    let mut first_input = String::new();
    io::stdin().read_line(&mut first_input).unwrap();
    let first_number: f64 = first_input.trim().parse().expect("Invalid number");

    // Prompt for the operation
    println!("Enter the operation (+, -, *, /):");
    let mut operation_input = String::new();
    io::stdin().read_line(&mut operation_input).unwrap();
    let operation = operation_input.trim();

    // Prompt for the second number
    println!("Enter the second number:");
    let mut second_input = String::new();
    io::stdin().read_line(&mut second_input).unwrap();
    let second_number: f64 = second_input.trim().parse().expect("Invalid number");

    let operation_enum = match operation {
        "+" => Operation::Add(first_number, second_number),
        "-" => Operation::Subtract(first_number, second_number),
        "*" => Operation::Multiply(first_number, second_number),
        "/" => Operation::Divide(first_number, second_number),
        _ => {
            println!("Invalid operation!");
            return;
        }
    };

    // Calculate and display the result
    let result = calculate(operation_enum);
    println!("The result is: {}", result);
}
enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Divide(f64, f64),
    Multiply(f64, f64),
}
fn calculate(op: Operation) ->f64
{
    match op {
        Operation::Add(x, y) => x + y,
        Operation::Subtract(x, y) => x - y,
        Operation::Divide(x, y) => {
            if y != 0.0 {
                x / y
            } else {
                panic!("Division by zero!");
            }
        }
        Operation::Multiply(x, y) => x * y,
    }
}

