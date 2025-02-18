
use std::io::{self, Write};

fn get_string(string_name: String) -> String {
    let mut value = String::new();
    println!("Please Enter {}:", string_name);
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin().read_line(&mut value).expect("Failed to read input");
    value
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
     (a/b).to_string()
}

fn parse_to_float64(a: String) -> f64 {
    let result: f64 = a.parse().expect("parse error");
    result
}


fn main() -> io::Result<()> {
    let mut first_number = String::new();
    let mut second_number = String::new();
    let mut operator = String::new();
    let mut result_number = String::new();
    println!("This tool takes 2 numbers and an operator, and calculate the result.");
    loop {
        first_number = get_string(String::from("First Number"));
        operator = get_string(String::from("Operater (+, -, *, /)"));
        second_number = get_string(String::from("Second Number"));
        match operator {
            "+" => result_number = add(parse_to_float64(first_number), parse_to_float64(second_number)),
        }

    }
}
