fn main() {
    let vec = vec![
        "nail", "shoe", "horse", "rider", "message", "battle", "kingdom",
    ];

    println!("{}",build_proverb(&vec));
}

fn build_proverb(list: &[&str]) -> String {
    let mut result = vec![];
    for (pos, ele) in list.iter().enumerate() {
        if pos == list.len() - 1 {
            result.push(format!("And all for the want of a {}.", list[0]));
        } else {
            result.push(format!(
                "For want of a {} the {} was lost.",
                ele,
                list[pos + 1]
            ));
        }
    }
    
    result.join("\n")
}
