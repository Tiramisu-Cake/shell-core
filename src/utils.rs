use crate::built_in_cmds::get_executable_file;
use crate::structs::{Config, SimpleCmd};
use std::fs::File;
use std::io::Error;
use std::path::Path;
use std::process::{Command, ExitStatus};

pub fn execute(args: &[String]) {
    let cmd = &args[0];
    let args = &args[1..];

    let file = get_executable_file(&cmd);
    if file.is_empty() {
        println!("{}: command not found", cmd);
        return;
    };
    let file = file.split("/").last().unwrap();
    Command::new(file).args(args).status();
}

pub fn overwrite_file(cmd: &str, input: &str, file: &Path) -> Result<ExitStatus, Error> {
    let file = File::create(file)?;
    let cmd = Command::new(cmd).arg(input).stdout(file).status()?;
    Ok(cmd)
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
