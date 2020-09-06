fn main() {
    println!("{}", "Removing Vowels from A String");

    let input = String::from("Hello Rust Programmers");

    println!("{}", remove_vowel(input));
}

fn remove_vowel(input: String) -> String {
    let mut result = String::new();
    for (_, c) in input.chars().enumerate() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => {},
            _ =>  result.push(c),
        }
    }
    result
}
