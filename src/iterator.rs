fn main() {
    println!("{}", "Learning Iterators");
    let languages = vec!["JS", "Java", "Perl", "Erlang", "Go", "Rust"];

    for (_pos, val) in languages.iter().enumerate() {
        println!("{}", val);
    }

    println!("{}", "Printin using Map");
    languages.iter().for_each(|x| println!("{}", x));
    

}
