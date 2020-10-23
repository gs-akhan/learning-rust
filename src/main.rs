#![feature(proc_macro_hygiene, decl_macro)]

// use rand::Rng;
// use std::cmp::Ordering;
// use std::io;
use rocket::*;

fn main() {
    println!("Welcome to Rust Guessing Game !!");
    rocket::ignite().mount("/", routes![hello]).launch();
}

#[get("/hello")]
fn hello() -> String {
    format!("Hello, This is my first API Rust!")
}

// let value: BigUint = 1.to_bigint().unwrap();

// loop {
//     println!("Please enter your Guess");

//     let mut guess = String::new();

//     io::stdin()
//         .read_line(&mut guess)
//         .expect("Failed to read line");

//     let guess: u32 = match guess.trim().parse() {
//         Ok(num) => num,
//         Err(_) => continue,
//     };

//     println!("You guessed : {}", guess);

//     let secret_number = rand::thread_rng().gen_range(1, 100);

//     println!("Secret is {} ", secret_number);

//     match guess.cmp(&secret_number) {
//         Ordering::Less => println!("Too small"),
//         Ordering::Greater => println!("Too big"),
//         Ordering::Equal => {
//             println!("You win");
//             break;
//         }
//     }
// }
