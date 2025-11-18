use codecrafters_shell::built_in_cmds::*;
use codecrafters_shell::parser::*;
use codecrafters_shell::redirect::*;
use codecrafters_shell::structs::TargetFile;
use codecrafters_shell::structs::*;
use codecrafters_shell::utils::*;
use libc::STDIN_FILENO;
use libc::STDOUT_FILENO;
use libc::{close, dup2};
use nix::sys::wait::{waitpid, WaitStatus};
use nix::unistd::{fork, pipe, ForkResult};
use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::os::fd::IntoRawFd;
use std::os::unix::io::AsRawFd;
use std::process::exit;

fn open_truncate_file(path: &str) -> io::Result<File> {
    OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
}

fn open_append_file(path: &str) -> io::Result<File> {
    OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(path)
}

fn open_and_get_fd(target_file: &TargetFile) -> File {
    let file;
    if target_file.append {
        file = open_append_file(&target_file.path).expect("failed to open redirection file");
    } else {
        file = open_truncate_file(&target_file.path).expect("failed to open redirection file");
    }
    file
}

fn run_simplecmd(cmd: &SimpleCmd) {
    let args = &cmd.args;
    let stdout = &cmd.stdout;
    let stderr = &cmd.stderr;

    let cmd = &args[0];

    let _guard_stdout;
    let _guard_stderr;

    match stdout {
        StreamTarget::Terminal => (),
        StreamTarget::File(target_file) => {
            let file = open_and_get_fd(target_file);
            let file_fd = file.as_raw_fd();
            io::stdout().flush().ok();
            _guard_stdout = FdRedirectGuard::new(1, file_fd);
        }
    }

    match stderr {
        StreamTarget::Terminal => (),
        StreamTarget::File(target_file) => {
            let file = open_and_get_fd(target_file);
            let file_fd = file.as_raw_fd();
            io::stdout().flush().ok();
            _guard_stderr = FdRedirectGuard::new(2, file_fd);
        }
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
        let config = parse_pipeline(&tokenize(input.trim()));
        if config.len() <= 1 {
            run_simplecmd(&config[0]);
            continue;
        }
        let (read_end, write_end) = pipe().unwrap();
        let write_fd = write_end.into_raw_fd();
        let read_fd = read_end.into_raw_fd();
        let fork1 = unsafe { fork() }.unwrap();
        match fork1 {
            ForkResult::Parent { child: pid1 } => {
                unsafe { close(write_fd) };
                let fork2 = unsafe { fork() }.unwrap();
                match fork2 {
                    ForkResult::Parent { child: pid2 } => {
                        unsafe { close(read_fd) };
                        let _status1 = waitpid(pid1, None);
                        let _status2 = waitpid(pid2, None);
                    }
                    ForkResult::Child => {
                        unsafe { dup2(read_fd, STDIN_FILENO) };
                        unsafe { close(read_fd) };
                        run_simplecmd(&config[1]);
                        exit(0);
                    }
                }
            }
            ForkResult::Child => {
                unsafe { close(read_fd) };
                unsafe { dup2(write_fd, STDOUT_FILENO) };
                unsafe { close(write_fd) };
                run_simplecmd(&config[0]);
                exit(0);
            }
        }
    }
}
