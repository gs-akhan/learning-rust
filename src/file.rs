use std::fs;

fn main() {
    println!("This is is file reading");

    let contents = fs::read_to_string("../static/poem.txt")
        .expect("Error in reading the file");
    println!("{}", contents);
}