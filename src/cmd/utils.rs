use crate::redirect::FdRedirectGuard;
use crate::structs::{StreamTarget, TargetFile};
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{Error, Write};
use std::os::fd::AsRawFd;
use std::os::unix::fs::PermissionsExt;
use std::os::unix::process::CommandExt;
use std::path::Path;
use std::process::{Command, ExitStatus};
use std::{env, io};

pub fn execute(args: &[String]) -> i32 {
    let cmd = &args[0];
    let args = &args[1..];

    let file = get_executable_file(&cmd);
    if file.is_empty() {
        println!("{}: command not found", cmd);
        return 1;
    };
    match Command::new(&file).arg0(cmd).args(args).status() {
        Ok(status) => match status.code() {
            Some(code) => return code,
            None => return 2,
        },
        Err(e) => {
            eprintln!("{file}: {e}");
            return 1;
        }
    }
}

pub fn get_executable_file(cmd: &str) -> String {
    let path;
    match env::var("PATH") {
        Ok(path_var) => path = path_var,
        Err(e) => {
            eprintln!("Couldn't read PATH: {e}");
            return "".to_string();
        }
    }
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

pub fn overwrite_file(cmd: &str, input: &str, file: &Path) -> Result<ExitStatus, Error> {
    let file = File::create(file)?;
    let cmd = Command::new(cmd).arg(input).stdout(file).status()?;
    Ok(cmd)
}

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

fn open_file(target_file: &TargetFile) -> io::Result<File> {
    let file;
    if target_file.append {
        file = open_append_file(&target_file.path)?;
    } else {
        file = open_truncate_file(&target_file.path)?;
    }
    Ok(file)
}

pub fn match_std(std: &StreamTarget, target_fd: i32) -> Result<Option<FdRedirectGuard>, Error> {
    match std {
        StreamTarget::Terminal => Ok(None),
        StreamTarget::File(target_file) => {
            let file = open_file(target_file)?;
            let file_fd = file.as_raw_fd();
            io::stdout().flush().ok();
            match FdRedirectGuard::new(target_fd, file_fd) {
                Ok(guard) => Ok(Some(guard)),
                Err(e) => Err(e),
            }
        }
    }
}
