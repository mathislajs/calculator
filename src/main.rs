use std::io::{stdin, stdout, Write};

fn main() {
    let mut a = String::new();
    let mut b = String::new();
    let mut operator = String::new();
    print!("Input the first number: ");
    stdout().flush().expect("Couldn't flush statement (1).");
    stdin().read_line(&mut a).expect("Couldn't read the first number.");
    print!("Input the second number: ");
    stdout().flush().expect("Couldn't flush statement (2).");
    stdin().read_line(&mut b).expect("Couldn't read the second number.");
    print!("Input the operation (+, -, *, /): ");
    stdout().flush().expect("Couldn't flush statement (operation).");
    stdin().read_line(&mut operator).expect("Couldn't read the operation.");

    let a: f32 = a.trim().parse().unwrap();
    let b: f32 = b.trim().parse().unwrap();
    let operator: char = operator.trim().chars().next().unwrap();
    
    let operators = String::from("+-/*");

    if !operators.contains(operator){
        panic!("Invalid operator.");
    }

    let mut result: f32 = 1.0;

    if operator =='+' {
        result = a + b;
    }
    else if operator == '-' {
        result = a - b;
    }
    else if operator == '*' {
        result = a * b;
    }
    else if operator == '/' {
        result = a / b;
    }
    println!("The result of {} {} {} is: {}.", a, operator, b, result);
}
