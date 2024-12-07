use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!("Usage: calculator <operation> <number1> <number2>");
        eprintln!("Operations: add, subtract, multiply, divide");
        return;
    }

    let operation = &args[1];
    let num1: f64 = match args[2].parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Error: {} is not a valid number.", args[2]);
            return;
        }
    };
    let num2: f64 = match args[3].parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Error: {} is not a valid number.", args[3]);
            return;
        }
    };

    let result = match operation.as_str() {
        "add" => num1 + num2,
        "subtract" => num1 - num2,
        "multiply" => num1 * num2,
        "divide" => {
            if num2 == 0.0 {
                eprintln!("Error: Division by zero is not allowed.");
                return;
            }
            num1 / num2
        }
        _ => {
            eprintln!("Error: Unknown operation '{}'.", operation);
            eprintln!("Supported operations: add, subtract, multiply, divide");
            return;
        }
    };

    println!("Result: {}", result);
}
