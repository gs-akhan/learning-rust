fn main() {

    //String in rust has three parts 
    // 1. pointer to memory that holds the content of the string
    // 2. length - Is the memory in bytes the content of the string is using.
    // 3. capacity - The total memory in bytes the string has recieved from operating system.

    let s1 = String::from("Tea !!");

    //If you don;t use the clone then the 
    let s2 = s1.clone();
    println!("{} {}", s1, s2);


    let mut name = String::from("Azhar");

    print_size(&name);
    add_str(&mut name);
    println!("Origanal String {} : ", name);


}

fn print_size(str : &String) {
    println!("{}", str.len());
}

fn add_str(str : &mut String) {
    str.push_str(" Thank you");
}
