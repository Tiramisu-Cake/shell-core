#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::exit;

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut cmd = String::new();
        io::stdin().read_line(&mut cmd).unwrap();
        let cmd = cmd.trim().to_owned();
        if cmd == "exit 0" {
            exit(0);
        }
        if cmd.starts_with("echo") {
            println!("{}", &cmd[5..]);
        } else {
            println!("{}: command not found", cmd);
        }
    }
}
