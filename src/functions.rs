
extern crate rand;

use rand::Rng;

fn add_two(a:i32, b : i32) -> i32{
    
    let x = {
        let y = 100;
        y
    };
    a + b + x
}

fn add_random(input: i32) -> i32 {
    
    i + rand::thread_rng().gen_range(1,100);
}