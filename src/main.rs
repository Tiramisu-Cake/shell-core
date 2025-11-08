#[allow(unused_imports)]
use std::io::{self, Write};
use std::{
    env::{self, VarError},
    fs,
    os::unix::fs::PermissionsExt,
    path::Path,
    process::exit,
    process::Command,
};

struct Config {
    cmd: String,
    args: Option<String>,
}

impl Config {
    fn build(input: String) -> Config {
        let mut parts = input.trim().splitn(2, " ");

        let cmd = parts.next().unwrap().to_string();
        let args = parts.next().map(|s| s.trim().to_string());

        Config { cmd, args }
    }
}

fn echo_cmd(args: &Option<String>) {
    let Some(args) = args else {
        println!("");
        return;
    };
    println!("{}", args);
}

fn type_cmd(cmd: &Option<String>) {
    let Some(cmd) = cmd else {
        println!("");
        return;
    };
    if is_built(&cmd) {
        println!("{} is a shell builtin", cmd);
        return;
    }

    let file = get_executable_file(&cmd);
    if file.len() > 0 {
        println!("{} is {}", cmd, file);
        return;
    } else {
        println!("{}: not found", cmd);
    }
}

fn get_executable_file(cmd: &str) -> String {
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

fn execute(config: Config) {
    let cmd = &config.cmd;
    let args = &config.args;

    let file = get_executable_file(&cmd);
    if file.is_empty() {
        println!("{}: command not found", cmd);
        return;
    };
    let file = file.split("/").last().unwrap();
    if let Some(args) = args {
        Command::new(file).args(args.split_whitespace()).status();
    } else {
        Command::new(file).status();
    }
}

fn is_built(cmd: &str) -> bool {
    let built_in = ["echo", "exit", "type"];
    return built_in.contains(&cmd);
}

fn read_path() -> Result<String, VarError> {
    env::var("PATH")
}

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(1) => {
                println!("");
                continue;
            }
            _ => (),
        }
        let config = Config::build(input);

        let cmd = &config.cmd;
        match cmd.as_str() {
            "echo" => echo_cmd(&config.args),
            "type" => type_cmd(&config.args),
            "exit" => exit(0),
            _ => execute(config),
        }
    }
}
