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
use nix::unistd::Pid;
use nix::unistd::{fork, pipe, ForkResult};
use rustyline::config::Configurer;
use rustyline::history::FileHistory;
use rustyline::Editor;
use std::env;
use std::fs::{File, OpenOptions};
use std::io::Error;
use std::io::{self, Write};
use std::os::fd::IntoRawFd;
use std::os::unix::io::AsRawFd;
use std::process::exit;
use std::slice::Iter;

use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

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

fn open_file(target_file: &TargetFile) -> File {
    let file;
    if target_file.append {
        file = open_append_file(&target_file.path).expect("failed to open redirection file");
    } else {
        file = open_truncate_file(&target_file.path).expect("failed to open redirection file");
    }
    file
}

fn match_std(std: &StreamTarget, target_fd: i32) -> Result<Option<FdRedirectGuard>, Error> {
    match std {
        StreamTarget::Terminal => Ok(None),
        StreamTarget::File(target_file) => {
            let file = open_file(target_file);
            let file_fd = file.as_raw_fd();
            io::stdout().flush().ok();
            match FdRedirectGuard::new(target_fd, file_fd) {
                Ok(guard) => Ok(Some(guard)),
                Err(e) => Err(e),
            }
        }
    }
}

fn run_builtin_cmd(cmd: &SimpleCmd) {}

fn run_simplecmd(state: &mut ShellState, cmd: &SimpleCmd) {
    let args = &cmd.args;
    let stdout = &cmd.stdout;
    let stderr = &cmd.stderr;

    let cmd = &args[0];

    let _guard_stdout = match_std(stdout, 1);
    let _guard_stderr = match_std(stderr, 2);

    match cmd.as_str() {
        "cd" => cd_cmd(&args[1..]),
        "echo" => echo_cmd(&args[1..]),
        "type" => type_cmd(&args[1..]),
        "exit" => exit(0),
        "pwd" => pwd_cmd(&args[1..]),
        "history" => history_cmd(state, &args[1..]),
        _ => execute(&args),
    }
}

// current main with rustyline
fn main() {
    let mut state = ShellState::new();
    let _ = state.editor.set_history_ignore_dups(false);

    if let Ok(hist_path) = env::var("HISTFILE") {
        let _ = state.editor.load_history(&hist_path);
    }

    loop {
        let line = state.editor.readline("$ ");
        match line {
            Ok(input) => {
                let trimmed = input.trim();
                if !trimmed.is_empty() {
                    state.editor.add_history_entry(trimmed);
                }

                let config = parse_pipeline(&tokenize(input.trim()));
                if config.len() <= 1 {
                    run_simplecmd(&mut state, &config[0]);
                    continue;
                }

                let mut it = config.iter();
                let cmd1 = it.next().unwrap();
                let cmd2 = it.next();
                run_pipeline(&mut state, &mut it, cmd1, cmd2, None, Vec::new());
            }
            Err(ReadlineError::Interrupted) => {
                println!("^C");
                continue;
            }
            Err(ReadlineError::Eof) => {
                println!("^D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}

// old main without rustyline
// fn main_old() {
//     let mut state = ShellState::new();
//     loop {
//         print!("$ ");
//         io::stdout().flush().unwrap();
//         let mut input = String::new();
//         match io::stdin().read_line(&mut input) {
//             Ok(1) => {
//                 // one symbol entered (i.e. /n), so we print a new line and go to the next
//                 // iteration
//                 println!("");
//                 continue;
//             }
//             _ => (),
//         }
//         state.history.push(input.trim().to_string());
//         let config = parse_pipeline(&tokenize(input.trim()));
//         if config.len() <= 1 {
//             run_simplecmd(&mut state, &config[0]);
//             continue;
//         }
//
//         let mut it = config.iter();
//         let cmd1 = it.next().unwrap();
//         let cmd2 = it.next();
//         run_pipeline(&mut state, &mut it, cmd1, cmd2, None, Vec::new());
//     }
// }

fn run_pipeline(
    mut state: &mut ShellState,
    mut cmds: &mut Iter<'_, SimpleCmd>,
    cmd1: &SimpleCmd,
    cmd2: Option<&SimpleCmd>,
    read_fd: Option<i32>,
    mut pids: Vec<Pid>,
) {
    match cmd2 {
        None => {
            match read_fd {
                None => (),
                Some(read_fd) => {
                    let fork = unsafe { fork() }.unwrap();
                    match fork {
                        ForkResult::Parent { child: pid } => {
                            pids.push(pid);
                            unsafe { close(read_fd) };
                            let mut waitpids = Vec::new();
                            for pid in pids {
                                waitpids.push(waitpid(pid, None));
                            }
                            // let _status = waitpid(pid, None);
                        }
                        ForkResult::Child => {
                            unsafe {
                                dup2(read_fd, STDIN_FILENO);
                                close(read_fd);
                            };
                            run_simplecmd(&mut state, cmd1);
                            exit(0);
                        }
                    }
                }
            }
        }
        Some(cmd) => {
            let (read_end, write_end) = pipe().unwrap();
            let write_fd = write_end.into_raw_fd();
            let next_read_fd = read_end.into_raw_fd();

            let next_fork = unsafe { fork() }.unwrap();
            match next_fork {
                ForkResult::Parent { child: pid } => {
                    unsafe {
                        match read_fd {
                            None => (),
                            Some(read_fd) => {
                                close(read_fd);
                            }
                        }
                        close(write_fd);
                    };
                    pids.push(pid);
                    let next_cmd = cmds.next();
                    run_pipeline(
                        &mut state,
                        &mut cmds,
                        cmd,
                        next_cmd,
                        Some(next_read_fd),
                        pids,
                    );
                }
                ForkResult::Child => {
                    unsafe {
                        close(next_read_fd);
                        match read_fd {
                            None => (),
                            Some(x) => {
                                dup2(x, STDIN_FILENO);
                                close(x);
                            }
                        }
                        dup2(write_fd, STDOUT_FILENO);
                        close(write_fd);
                    };
                    run_simplecmd(&mut state, cmd1);
                    exit(0);
                }
            }
        }
    }
}
