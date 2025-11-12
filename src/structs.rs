use crate::parser2::*;

pub struct Config {
    pub cmd: String,
    pub args: Vec<String>,
}

impl Config {
    pub fn build(input: String) -> Config {
        let mut parts = input.trim().splitn(2, " ");

        let cmd = parts.next().unwrap().to_string();
        let input = parts.next().map(|s| tokenize(s.trim()));
        let mut args = Vec::new();
        if let Some(input_unpacked) = input {
            args = input_unpacked;
        } else {
        }

        Config { cmd, args }
    }
}
