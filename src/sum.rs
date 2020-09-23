pub fn square_of_sum(n: u32) -> u32 {
    let mut sum = 0;
    for i in 0..n+1{
        sum  = sum + i;
    }
    sum * sum
}

pub fn sum_of_squares(n: u32) -> u32 {
    let mut sum = 0;
    for i in 0..n+1 {
        sum  = sum + (i * i);
    }
    sum
}

fn main() {
    let res = square_of_sum(10) - sum_of_squares(10);
    println!("{}", res);
}
