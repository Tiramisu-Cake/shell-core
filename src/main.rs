use codecrafters_shell::built_in_cmds::*;
use codecrafters_shell::redirect::*;
use codecrafters_shell::structs::*;
use codecrafters_shell::utils::*;
use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::os::unix::io::AsRawFd;
use std::process::exit;

fn open_truncate_file(path: &str) -> io::Result<File> {
    OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
}
fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(1) => {
                // one symbol entered (i.e. /n), so we print a new line and go to the next
                // iteration
                println!("");
                continue;
            }
            _ => (),
        }
        let config = SimpleCmd::build(&input);
        let args = config.args;
        let redirs = config.redirs;

        let cmd = &args[0];

        let _guard;

        if !redirs.is_empty() {
            let file = open_truncate_file(&redirs[0]).expect("failed to open redirection file");
            let file_fd = file.as_raw_fd();
            io::stdout().flush().ok();
            _guard = FdRedirectGuard::new(1, file_fd);
        }

        match cmd.as_str() {
            "cd" => cd_cmd(&args[1..]),
            "echo" => echo_cmd(&args[1..]),
            "type" => type_cmd(&args[1..]),
            "exit" => exit(0),
            "pwd" => pwd_cmd(&args[1..]),
            _ => execute(&args),
        }
    }
}
