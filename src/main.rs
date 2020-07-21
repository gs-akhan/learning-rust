extern crate rand;

use std::io;
use rand::Rng;

fn main() {
    
    println!("Welcome to Rust Guessing Game !!");

    println!("Please enter your Guess"); 
    
    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed : {}", guess);

    let secret_number = rand::thread_rng().gen_range(1,100);
    
    println!("{}", secret_number);
}
