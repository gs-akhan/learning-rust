fn main() {
    println!("{}", "Reversing the String ");
    let str = "Azhar is here";
    let mut result = vec![];
    for (_, c) in str.chars().enumerate() {
        result.insert(0, c.to_string());
    }
    println!("{}", result.join(""));
}
