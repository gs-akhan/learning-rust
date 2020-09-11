fn main() {
    println!("Hello Operators");

    let mut array:Vec<i32> = vec![1,2,3,4,5,6,7];

    array.push(8);
    array.iter()
        .map(|x| x * 2)
        .filter(|x| *x > 4)
        .for_each(|x| println!("{}", x));   
}