const MAX_POINTS : u32 = 100_000;
fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    //shadowing 

    let x = 100;
    let x = 100 + 1;
    let x = x * 2;

    //This way of declaring new variables with same name as previous variables
    //is called as shadowing. 
    //Specially used in place where we want to use same name 
    //for variables after assigning them different types.


}