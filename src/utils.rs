use crate::built_in_cmds::get_executable_file;
use crate::structs::Config;
use std::process::Command;

pub fn execute(config: Config) {
    let cmd = &config.cmd;
    let args = &config.args;

    let file = get_executable_file(&cmd);
    if file.is_empty() {
        println!("{}: command not found", cmd);
        return;
    };
    let file = file.split("/").last().unwrap();
    Command::new(file).args(args).status();
}

pub mod env {

    use std::env;
    use std::env::VarError;

    pub fn read_path() -> Result<String, VarError> {
        env::var("PATH")
    }

    pub fn read_home() -> Result<String, VarError> {
        env::var("HOME")
    }
}
