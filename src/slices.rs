fn main() {
    println!("Hello slices");

    let name:String = String::from("I am here");

    let bytes = name.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        println!("{} , {}", i, &item);
    }

    let newname = String::from("This is nice");

    for (_i, c) in newname.chars().enumerate() {
        println!("{}", c);
    }


    let rollno = "3123sfc";

    println!("{}", rollno);
    
    let data = "Azhar";
    let str = data.to_string();
    greet(str);


    //Substring;

    let complete_string = String::from("Rust is a Symtems Programming Language");

    let sub_str = &complete_string[0..10];
    
    sub_str.to_string().push_str("More Data");


    // let new_owner = sub_str;

    println!("Substring : {} ", sub_str);
    println!("Orig String {}", complete_string);

}


fn greet(msg:String) {
    println!("Mr/Ms, {}", msg);
}