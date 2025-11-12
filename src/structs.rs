use crate::parser2::*;

pub struct Config {
    pub cmd: Vec<String>,
    /*
    pub cmd: String,
    pub args: Vec<String>,
    */
}

impl Config {
    pub fn build(input: String) -> Config {
        let cmd = tokenize(input.trim());

        Config { cmd }
    }
}
