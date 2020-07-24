extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    
    println!("Welcome to Rust Guessing Game !!");

    loop {
        println!("Please enter your Guess"); 
    
            let mut guess = String::new();

            io::stdin().read_line(&mut guess)
                .expect("Failed to read line");

            let guess:u32 = guess.trim().parse().expect("Enter valid number");

            println!("You guessed : {}", guess);

            let secret_number = rand::thread_rng().gen_range(1,100);

            println!("Secret is {} ", secret_number);

            match guess.cmp(&secret_number) {
                Ordering::Less => println!("Too small"),
                Ordering::Greater => println!("Too big"),
                Ordering::Equal => println!("You wil"),
            }
    }
}
