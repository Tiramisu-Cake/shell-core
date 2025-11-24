use crate::{
    structs::ShellState,
    utils::{env::*, overwrite_file},
};
use rustyline::history::{FileHistory, History};
use rustyline::Editor;
use std::{
    env::{current_dir, set_current_dir},
    fs,
    fs::File,
    fs::OpenOptions,
    io::Write,
    os::unix::fs::PermissionsExt,
    path::Path,
};

const CMDS: [&str; 6] = ["cd", "echo", "exit", "type", "pwd", "history"];

pub fn is_builtin(cmd: &str) -> bool {
    return CMDS.contains(&cmd);
}

pub fn get_executable_file(cmd: &str) -> String {
    let path = read_path().unwrap();
    let paths_dirs: Vec<&str> = path.split(':').collect();
    for dir in paths_dirs {
        let file = format!("{}/{}", dir, cmd);
        let file_path = Path::new(&file);
        if let Ok(metadata) = fs::metadata(&file_path) {
            let exec_rights = metadata.permissions().mode();
            if exec_rights & 0o100 != 0 {
                return file;
            }
        } else {
            continue;
        };
    }
    return "".to_string();
}

pub fn echo_cmd(args: &[String]) {
    if args.is_empty() {
        println!("");
        return;
    };
    println!("{}", args.join(" "));
    return;
}
pub fn history_cmd(state: &mut Editor<(), FileHistory>, args: &[String]) {
    let mut args = args.iter();
    let bound = match args.next() {
        Some(ref arg) => match arg.parse::<usize>() {
            Ok(val) => val,
            Err(_) => match arg.as_str() {
                "-r" => {
                    if let Some(history_path) = args.next() {
                        state.load_history(history_path);
                        return;
                    } else {
                        return;
                    }
                }
                "-w" => {
                    if let Some(history_path) = args.next() {
                        let history = state.history_mut().iter();
                        let mut history_file = File::create(history_path).unwrap();
                        for x in history {
                            writeln!(history_file, "{}", x);
                        }
                        return;
                    } else {
                        return;
                    }
                }
                "-a" => {
                    if let Some(history_path) = args.next() {
                        let history = state.history_mut().iter();
                        let mut history_file = OpenOptions::new()
                            .write(true)
                            .create(true)
                            .append(true)
                            .open(history_path)
                            .unwrap();
                        for x in history {
                            writeln!(history_file, "{}", x);
                        }
                        return;
                    } else {
                        return;
                    }
                }
                _ => {
                    return;
                }
            },
        },
        None => {
            for (i, line) in state.history().iter().enumerate() {
                println!("  {} {}", i + 1, line);
            }
            return;
        }
    };
    let size = state.history().len();

    for (i, line) in state.history().iter().enumerate() {
        if i >= size - bound {
            println!("  {} {}", i + 1, line);
        }
    }
}
pub fn type_cmd(args: &[String]) {
    if args.is_empty() {
        println!("");
        return;
    };
    if args.len() > 1 {
        println!("{}: not found", args.join(" "));
    }
    let cmd = args[0].clone();
    if is_builtin(&cmd) {
        println!("{} is a shell builtin", cmd);
        return;
    }
    let file = get_executable_file(&cmd);
    if file.len() > 0 {
        println!("{} is {}", cmd, file);
        return;
    }
    println!("{}: not found", cmd);
}
pub fn pwd_cmd(args: &[String]) {
    let wd = current_dir();
    match wd {
        Ok(path) => println!("{}", path.display()),
        Err(e) => println!("aaaaaaa"),
    }
}

pub fn cd_cmd(args: &[String]) {
    if args.is_empty() {
        return;
    }
    let mut path = args[0].to_string();
    if path == "~" {
        path = read_home().unwrap();
    }
    let cd = set_current_dir(&path);
    match cd {
        Ok(()) => (),
        Err(e) => println!("cd: {}: No such file or directory", path),
    }
}
