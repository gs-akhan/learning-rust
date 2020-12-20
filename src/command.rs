use std::process::Command;

fn main() {
    println!("Executing a command");

    let mut cmd = Command::new("ssh");
    cmd.arg("akhan-dev@10.0.0.31");

    match cmd.output() {
        Ok(o) => unsafe {
            println!("{}", String::from_utf8_unchecked(o.stdout));
        },
        Err(_e) => {
            println!("There was error");
        }
    }
}
