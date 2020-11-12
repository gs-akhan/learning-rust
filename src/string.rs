fn main() {
    println!("Working with the Strings");
    
    let my_name = "Azhar is HERE";

    println!("Lower Cased : {}", my_name.to_lowercase());

    let newname = my_name;

    println!("Orig String {}", newname);
    println!("Orig String {}", my_name);


    // Let String as a Objet

    let hello = String::from("Hello MR ");


    // Pass my Reference !!
    let new_hello = &hello;

    println!("{}", hello);
    
}