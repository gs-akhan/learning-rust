#[allow(unused_variables)]
#[allow(unused_mut)]

fn main() {
    println!("===== Matching Brackets =====");

    let str = "[{}]".to_string();

    let mut stack: Vec<char> = vec![];

    for (i, c) in str.chars().enumerate() {
        match c {
            '[' | '(' | '{' => {
                stack.push(c);
            }
            ']' | ')' | '}' => {
                if stack.len() == 0 {
                    println!("Not Balanced");
                }
                let item = stack.pop().unwrap();
                if item == '(' && c != ')' {
                    println!("Not Balanced");
                }
                if item == '{' && c != '}' {
                    println!("Not Balanced");
                }
                if item == '[' && c != ']' {
                    println!("Not Balanced");
                }
            }
            _ => {}
        }
    }

    println!("Balanced");
}
