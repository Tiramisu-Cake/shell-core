use super::utils::get_executable_file;
use crate::structs::ShellState;

use rustyline::history::History;
use std::env;
use std::{
    env::{current_dir, set_current_dir},
    fs::OpenOptions,
    io::Write,
};

const CMDS: [&str; 6] = ["cd", "echo", "exit", "type", "pwd", "history"];

pub fn is_builtin(cmd: &str) -> bool {
    return CMDS.contains(&cmd);
}

pub fn exit_cmd(state: &mut ShellState) -> i32 {
    if let Ok(ref hist_path) = env::var("HISTFILE") {
        match write_history(state, true, hist_path) {
            Ok(_) => return 0,
            Err(e) => {
                eprintln!("warning: failed to write history: {hist_path}: {e}");
                return 0;
            }
        }
    } else {
        return 0;
    }
}

pub fn echo_cmd(args: &[String]) -> i32 {
    if args.is_empty() {
        println!("");
    } else {
        println!("{}", args.join(" "));
    }
    return 0;
}
pub fn history_cmd(state: &mut ShellState, args: &[String]) -> i32 {
    let mut args = args.iter();
    let size = state.editor.history().len();
    let bound = match args.next() {
        Some(arg) => match arg.parse::<usize>() {
            Ok(val) => {
                if args.next().is_none() {
                    val
                } else {
                    eprintln!("history: usage: history [N]");
                    return 2;
                }
            }
            Err(_) => {
                if let Some(history_path) = args.next()
                    && args.next().is_none()
                {
                    return history_flags(state, arg, history_path);
                } else {
                    eprintln!("history: usage: history [N]");
                    return 2;
                }
            }
        },
        None => size,
    };
    let skip = size.saturating_sub(bound);

    for (i, line) in state.editor.history().iter().skip(skip).enumerate() {
        println!("  {} {}", skip + i + 1, line);
    }
    return 0;
}

fn write_history(
    state: &mut ShellState,
    append: bool,
    history_path: &str,
) -> Result<(), std::io::Error> {
    let mut history_file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(append)
        .open(history_path)?;

    if !append {
        state.history = 0;
    }
    for entry in state.editor.history().iter().skip(state.history) {
        writeln!(history_file, "{}", entry)?;
    }

    state.history = state.editor.history().len();
    Ok(())
}

fn history_flags(state: &mut ShellState, flag: &str, history_path: &str) -> i32 {
    match flag {
        "-r" => match state.editor.load_history(history_path) {
            Ok(_) => return 0,
            Err(e) => {
                eprintln!("history: {history_path} {e}");
                return 1;
            }
        },
        "-w" => match write_history(state, false, history_path) {
            Ok(_) => return 0,
            Err(e) => {
                eprintln!("history: {history_path} {e}");
                return 1;
            }
        },
        "-a" => match write_history(state, true, history_path) {
            Ok(_) => return 0,
            Err(e) => {
                eprintln!("history: {history_path} {e}");
                return 1;
            }
        },
        _ => {
            eprintln!("history: invalid argument: {flag}");
            2
        }
    }
}
pub fn type_cmd(args: &[String]) -> i32 {
    if args.is_empty() {
        eprintln!("type: usage: type NAME");
        return 2;
    };
    if args.len() > 1 {
        eprintln!("type: usage: type NAME");
        return 2;
    }
    let cmd = args[0].clone();
    if is_builtin(&cmd) {
        println!("{} is a shell builtin", cmd);
        return 0;
    }
    let file = get_executable_file(&cmd);
    if file.len() > 0 {
        println!("{} is {}", cmd, file);
        return 0;
    }
    eprintln!("{}: not found", cmd);
    return 1;
}
pub fn pwd_cmd(_args: &[String]) -> i32 {
    //TODO add options -L -P
    match current_dir() {
        Ok(path) => {
            println!("{}", path.display());
            return 0;
        }
        Err(e) => {
            eprintln!("pwd: {e}");
            return 1;
        }
    }
}

pub fn cd_cmd(args: &[String]) -> i32 {
    if args.is_empty() {
        return 0;
    }
    let mut path = args[0].to_string();
    if path == "~" {
        match env::var("HOME") {
            Ok(home) => path = home,
            Err(e) => {
                eprintln!("cd: {e}");
                return 1;
            }
        }
    }
    if let Err(e) = set_current_dir(&path) {
        let msg = match e.kind() {
            std::io::ErrorKind::NotFound => "No such file or directory",
            std::io::ErrorKind::PermissionDenied => "Permission denied",
            _ => "Error",
        };

        eprintln!("cd: {}: {}", path, msg);
        return 1;
    }

    0
}
