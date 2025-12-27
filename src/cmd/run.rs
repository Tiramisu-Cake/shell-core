use super::built_in_cmds::*;
use super::utils::*;
use crate::structs::{ShellState, SimpleCmd};

use libc::STDIN_FILENO;
use libc::STDOUT_FILENO;
use libc::{close, dup2};
use nix::sys::wait::waitpid;
use nix::unistd::Pid;
use nix::unistd::{ForkResult, fork, pipe};
use std::os::fd::IntoRawFd;
use std::process::exit;
use std::slice::Iter;

pub fn run_simplecmd(state: &mut ShellState, cmd: &SimpleCmd) -> i32 {
    if cmd.args.is_empty() {
        return 0;
    }
    let args = &cmd.args;
    let stdout = &cmd.stdout;
    let stderr = &cmd.stderr;

    let cmd = &args[0];

    let _stdout_guard = match match_std(stdout, 1) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("path: {e}");
            return 1;
        }
    };
    let _stderr_guard = match match_std(stderr, 2) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("path: {e}");
            return 1;
        }
    };

    match cmd.as_str() {
        "cd" => cd_cmd(&args[1..]),
        "echo" => echo_cmd(&args[1..]),
        "type" => type_cmd(&args[1..]),
        "exit" => {
            let exit_code = exit_cmd(state);
            exit(exit_code);
        }
        "pwd" => pwd_cmd(&args[1..]),
        "history" => history_cmd(state, &args[1..]),
        _ => execute(&args),
    }
}

pub fn run_pipeline(
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
                            let exit_code = run_simplecmd(&mut state, cmd1);
                            exit(exit_code);
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
