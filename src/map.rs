fn main() {
    println!("{}", "Learning Map");

    let names = vec!["azhar", "khan"];

    let upp_names:Vec<String> = names.iter().map(|x| x.to_uppercase()).collect();

    upp_names.iter().for_each(|x| println!("{}", x));

    
}
