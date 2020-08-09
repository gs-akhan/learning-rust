// Here we will learn about types.

// A scalar type represents a single value. Rust has four primary scalar
// types: integers, floating-point numbers, booleans, and characters. You’ll
// likely recognize these from other programming languages, but let’s
// jump into how they work in Rust.

fn main() {

    //i8, i16, i32, i64
    //u8, u16, u32, u64
    let age : i32 = 29;
    println!("{}", age);

    let product = age * 100;

    println!("{}", product);

    //booleans

    let is_new : bool = false;
    let is_old : bool = true;
    let _am = is_new || is_old;


    // characters 

    let c : char = 'A';
    // Rust’s char type represents a Unicode Scalar Value, which means
    // it can represent a lot more than just ASCII. 
    // Accented letters, Chinese/Japanese/Korean ideographs, emoji, and zero width spaces are
    // all valid char types in Rust. Unicode Scalar Values range from U+0000
    // to U+D7FF and U+E000 to U+10FFFF inclusive

    // More to follow
}