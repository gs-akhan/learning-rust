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

3. Iterate over vec

```
let data = vec!["Azhar", "Sheetal", "Ganga"];
data.iter().for_each(|x| println!("{}", x));
```
