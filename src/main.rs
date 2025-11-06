#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut cmd = String::new();
        io::stdin().read_line(&mut cmd).unwrap();
        if cmd == "exit 0" {
            println!("Exit with status 0 (success)");
            break;
        }
        println!("{}: command not found", cmd.trim());
    }
}
