#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::exit;

enum BuiltInCommand {
    Type,
    Echo,
    Exit,
}
enum Command {
    BuiltIn(BuiltInCommand),
    External(String),
}

fn parse_command(cmd: &str) -> Command {
    match cmd {
        c if c.starts_with("type") => Command::BuiltIn(BuiltInCommand::Type),
        c if c.starts_with("echo") => Command::BuiltIn(BuiltInCommand::Echo),
        c if c.starts_with("exit") => Command::BuiltIn(BuiltInCommand::Exit),
        other => Command::External(other.to_string()),
    }
}

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let cmd = parse_command(input.trim());
        match cmd {
            Command::BuiltIn(BuiltInCommand::Type) => {
                if let Some(last_cmd) = input.trim().split_whitespace().last() {
                    if last_cmd == "echo" || last_cmd == "type" || last_cmd == "exit" {
                        println!("{} is a shell builtin", last_cmd);
                    } else {
                        println!("{}: not found", last_cmd);
                    }
                }
            }
            Command::BuiltIn(BuiltInCommand::Echo) => println!("{}", &input[5..].trim()),
            Command::BuiltIn(BuiltInCommand::Exit) => exit(0),
            Command::External(ext_cmd) => println!("{}: command not found", ext_cmd),
        }
    }
}
