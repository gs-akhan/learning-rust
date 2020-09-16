fn main() {
    println!("{}", nth(200));
}

pub fn nth(n: u32) -> u32 {
    let mut primes = Vec::new();
    if n == 0 {
        return 2;
    }

    if n == 1 {
        return 3
    }
    
    'outer: for x in 4.. {
        for y in 3..x + 1 {
            if x == y {
                primes.push(x);
                if primes.len() as i32 == n as i32 {
                    return x;
                }
            }

            if x % y == 0 {
                continue 'outer;
            }
        }
    }
    0
}
