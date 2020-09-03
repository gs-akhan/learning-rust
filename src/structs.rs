fn main() {
    let name = String::from("We are in Structs");
    println!("{}", name);
    struct User {
        user_name: String,
        email: String,
        age: i32,
        active: bool,
    };

    let azhar = User {
        user_name: String::from("Azhar"),
        email: String::from("wolf@crazyman.com"),
        age: 29,
        active: true,
    };

    println!("Name is : {}", azhar.user_name);
    println!("Age is : {}", azhar.age);
    println!("email is : {}", azhar.email);
    println!("Active is : {}", azhar.active);
}
