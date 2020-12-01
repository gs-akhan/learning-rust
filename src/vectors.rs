fn main() {
    println!("Lets Learn vectors in this ");

    let mut names = vec!["Azhar", "Sheetal", "John", "Dimitry"];

    println!("Last Name is : {}", names.last().expect("hahah"));
    names.iter().for_each(|x| println!("{}", x));

    println!("Length is : {}", names.len());

    // unsafe {
    //     names.set_len(1);    
    // }

    // cloning a part of vector. clone
    let new_names = Vec::from_iter(names[1..2]).iter().clone();
}
