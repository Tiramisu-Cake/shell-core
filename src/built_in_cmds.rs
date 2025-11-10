use crate::utils::env::*;
use std::{
    env::{current_dir, set_current_dir},
    fs,
    os::unix::fs::PermissionsExt,
    path::Path,
};

const CMDS: [&str; 5] = ["cd", "echo", "exit", "type", "pwd"];

pub fn is_built(cmd: &str) -> bool {
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

pub fn echo_cmd(args: &Vec<String>) {
    if args.is_empty() {
        println!("");
        return;
    };
    println!("{}", args.join(" "));
}

pub fn type_cmd(cmd: &Vec<String>) {
    if cmd.is_empty() {
        println!("");
        return;
    };
    if cmd.len() > 1 {
        println!("{}: not found", cmd.join(" "));
    }
    let cmd = cmd[0].clone();
    if is_built(&cmd) {
        println!("{} is a shell builtin", cmd);
        return;
    }
    let file = get_executable_file(&cmd);
    if file.len() > 0 {
        println!("{} is {}", cmd, file);
        return;
    }
}
pub fn pwd_cmd() {
    let wd = current_dir();
    match wd {
        Ok(path) => println!("{}", path.display()),
        Err(e) => println!("aaaaaaa"),
    }
}

pub fn cd_cmd(path: &Vec<String>) {
    if path.is_empty() {
        return;
    }
    let mut path = path[0].to_string();
    if path == "~" {
        path = read_home().unwrap();
    }
    let cd = set_current_dir(&path);
    match cd {
        Ok(()) => (),
        Err(e) => println!("cd: {}: No such file or directory", path),
    }
}
