use rand::Rng;
use std::io::{self, Write};


fn get_input() -> String {
    let mut value = String::new();
    println!("Please Enter a number between 0 to 100 ");
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin().read_line(&mut value).expect("Failed to read input");
    value.trim().to_string()
}

fn is_int(a: &String) -> bool {
    a.parse::<i32>().is_ok()
}

fn parse_to_i32(a: String) -> i32 {
    let result: i32 = a.parse().expect("parse error");
    result
}

fn main() {
    loop{
        let mut rng = rand::thread_rng();
        let ans = rng.gen_range(0..100);
        loop{
            let input = get_input();
            if is_int(&input){
                if parse_to_i32(input) == ans{
                    println!("You win!!!");
                    break
                }else{
                    println!("Wrong, guess again");
                    if parse_to_i32(input)> ans{
                        
                    }

                }

            }else{
                println!("Must enter a number between 1 to 100")
            }
        }

    }

}
