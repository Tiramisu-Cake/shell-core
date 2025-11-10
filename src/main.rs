use codecrafters_shell::built_in_cmds::*;
use codecrafters_shell::structs::*;
use codecrafters_shell::utils::*;
use std::io::{self, Write};
use std::process::exit;

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(1) => {
                println!("");
                continue;
            }
            _ => (),
        }
        let config = Config::build(input);

        let cmd = &config.cmd;
        match cmd.as_str() {
            "cd" => cd_cmd(&config.args),
            "echo" => echo_cmd(&config.args),
            "type" => type_cmd(&config.args),
            "exit" => exit(0),
            "pwd" => pwd_cmd(),
            _ => execute(config),
        }
    }
}
