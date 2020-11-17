### Helpers 

1. How to read line from cli ?

```
let mut guess = String::new();
io::stdin().read_line(&mut guess)
.expect("Failed to read line");
```

2. Convert String to number 

```
let str = "10";
let num:i32 = str.parse().unwrap().expect(0);
// If not able parse then returns 0
```

// let value: BigUint = 1.to_bigint().unwrap();

// loop {
//     println!("Please enter your Guess");

//     let mut guess = String::new();

//     io::stdin()
//         .read_line(&mut guess)
//         .expect("Failed to read line");

//     let guess: u32 = match guess.trim().parse() {
//         Ok(num) => num,
//         Err(_) => continue,
//     };

//     println!("You guessed : {}", guess);

//     let secret_number = rand::thread_rng().gen_range(1, 100);

//     println!("Secret is {} ", secret_number);

//     match guess.cmp(&secret_number) {
//         Ordering::Less => println!("Too small"),
//         Ordering::Greater => println!("Too big"),
//         Ordering::Equal => {
//             println!("You win");
//             break;
//         }
//     }
// }