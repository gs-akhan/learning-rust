use std::io;
fn main() {
    
    println!("Welcome to Rust Guessing Game !!");

    println!("Please enter your Guess"); 
    
    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed : {}", guess);
}
