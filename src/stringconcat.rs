fn main() {
    println!("This is string concat");
    let str1 = "Hello";
    let str2 = "World";

    let final_string = format!("{} {}", str1, str2);

    println!("{}", final_string);
}