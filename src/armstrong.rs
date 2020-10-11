fn main() {
    println!("{}", "==== Armstrong Numbers ====");

    let num = "153";

    let str = num.to_string();

    let count = 3;
    let mut result = 0;
    for (_, c) in str.chars().enumerate() {
        let num: i64 = c.to_string().parse().expect("Failed to convert");
        println!("{}", num);
        result = result + num.pow(count);
    }

    println!("{}", result);
}

pub fn is_armstrong() -> bool {
    true
}
