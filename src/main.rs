use std::io;

fn main() {
    loop {
        println!("Guess the Number!");

        let mut user_input = String::new();
        let user_input = match io::stdin().read_line(&mut user_input) {
            Ok(_) => user_input.trim(),
            Err(error) => {
                println!("Could not read input: {}", error.to_string());
                break;
            }
        };

        let guessed_number = match user_input.parse::<i32>() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input valid number");
                continue;
            }
        };

        println!("You Guessed {}", guessed_number);
    }
}
