fn main() {
    println!("---- Learning Fold ----");
    let a: Vec<i32> = (1..10).collect();
    let result = a.iter().fold(0, |a, b| a + b);
    println!("Final Result is {}", result);
}
