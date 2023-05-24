use colored::Colorize;
use rand::{thread_rng, Rng};
use std::cmp::Ordering;
use std::io;

fn main() {
    let mut rng = thread_rng();
    let answer: i32 = rng.gen_range(0..101);

    println!("the number is {}", answer);

    loop {
        println!("Guess the Number!");

        let mut user_input = String::new();
        let user_input = match io::stdin().read_line(&mut user_input) {
            Ok(_) => user_input.trim(),
            Err(error) => {
                println!("Could not read input: {}", error.to_string().red());
                break;
            }
        };

        let guessed_number = match user_input.parse::<i32>() {
            Ok(num) => num,
            Err(_) => {
                println!("{}", "Please input valid number".red());
                continue;
            }
        };

        match guessed_number.cmp(&answer) {
            Ordering::Less => println!("{}", "Too Small!".yellow().bold()),
            Ordering::Greater => println!("{}", "Too Big!".yellow().bold()),
            Ordering::Equal => {
                println!("{}", "You Win!".green().bold());
                break;
            }
        }
    }
}
