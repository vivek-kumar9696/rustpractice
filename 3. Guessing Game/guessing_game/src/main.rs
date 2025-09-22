use std::io;
use rand::Rng;
use std::cmp::Ordering;
use colored::{self, Colorize};

fn main() {
    println!("Guess The Number!");

    let secret_number=rand::rng().random_range(1..=100);
    println!("Secret Number is: {}", secret_number);

    loop {
        
        println!("Please input your guess!");
        let mut guess = String::new();
        
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        let guess: u32= match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
                }
            };

        println!("You Guessed: {}",guess);

        match guess.cmp(&secret_number) 
        {
            Ordering::Less => println!("{}","Too Small!".red()),
            Ordering::Greater => println!("{}","Too Big!".red()),
            Ordering::Equal => {
                println!("{}","You Win!".green());
                break;
            }
        
        }
    
    }
}