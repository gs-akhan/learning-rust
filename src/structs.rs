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

    let shtl = User {
        user_name : String::from("Shtl"),
        email : String::from("ahha@gmail.com"),
        ..azhar
    };


    println!("S Name {}",shtl.user_name);

    println!("Name is : {}", azhar.user_name);
    println!("Age is : {}", azhar.age);
    println!("email is : {}", azhar.email);
    println!("Active is : {}", azhar.active);
}
