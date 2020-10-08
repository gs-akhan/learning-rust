fn main() {
    println!("Improved Prime factors");

    let mut value = 8;
    let mut result = vec![];

    for factor in 2.. {
        if value < 2 {
            break;
        }

        while value % factor == 0 {
            value = value / factor;
            result.push(factor);
        }
    }

    result.iter().for_each(|x| println!("{}", x));
}
