use std::io::{self, Write};

fn get_string(string_name: String) -> String {
    let mut value = String::new();
    println!("Please Enter {}:", string_name);
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin().read_line(&mut value).expect("Failed to read input");
    value.trim().to_string()
}

fn add(a: f64, b:f64) -> String{
    (a+b).to_string()
}

fn substract(a: f64, b:f64) -> String{
    (a-b).to_string()
}

fn multiply(a: f64, b:f64) -> String{
    (a*b).to_string()
}

fn divide(a: f64, b:f64) -> String{
    if a == 0.0 {
        return String::from("Error: dividion by zero.")
    }
     (a/b).to_string()
}

fn parse_to_float64(a: String) -> f64 {
    let result: f64 = a.parse().expect("parse error");
    result
}

fn is_number(a: &String) -> bool {
    a.parse::<f64>().is_ok()
}


fn main() -> io::Result<()> {

    println!("This tool takes 2 numbers and an operator, and calculate the result.");
    loop {
        let first_number = get_string(String::from("First Number"));
        let operator = get_string(String::from("Operater (+, -, *, /)"));
        let second_number = get_string(String::from("Second Number"));
        if is_number(&first_number) && is_number(&second_number){
            match operator.as_str() {
                "+" => println!("Answer: {}", add(parse_to_float64(first_number), parse_to_float64(second_number))),
                "-" => println!("Answer: {}", substract(parse_to_float64(first_number), parse_to_float64(second_number))),
                "*" => println!("Answer: {}", multiply(parse_to_float64(first_number), parse_to_float64(second_number))),
                "/" => println!("Answer: {}", divide(parse_to_float64(first_number), parse_to_float64(second_number))),
                _ => println!("Unknown operator {}", operator)
            }
        }else {
            println!("Invalid inputs: {}{}{}", first_number, operator, second_number);
        }

    }
}