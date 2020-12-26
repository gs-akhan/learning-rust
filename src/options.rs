fn main() {
    println!("==== This is options ====");
    println!("{:?}", get_result());

    let data = get_result().unwrap();
    println!("This is unwrapped : {}", data);

    let data = get_options();
    println!("This is options function : {} ", data.unwrap());

    match get_options() {
        Some(x) => println!("{}", x),
        None => println!("We found nothing"),
    }

    let names = vec!["Azhar", "khan"];
    names.iter().for_each(|x| println!("{}", x));
}

fn get_result() -> Result<String, Box<dyn std::error::Error>> {
    Ok(String::from("This is from result"))
}

fn get_options() -> Option<String> {
    Some(String::from("This is from the Optins"))
}
