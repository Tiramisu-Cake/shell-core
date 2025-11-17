use std::str;

use crate::parser::*;

pub struct Config {
    pub cmd: Vec<String>,
    /*
    pub cmd: String,
    pub args: Vec<String>,
    */
}

// impl Config {
//     pub fn build(input: String) -> Config {
//         let cmd = tokenize(input.trim());
//
//         Config { cmd }
//     }
// }

pub struct AST {
    pub op: Node,
    pub kids: Option<Box<AST>>,
}

pub enum Node {
    Simple,
    Pipeline,
    Sequence,
    Group,
}

pub struct SimpleCmd {
    pub args: Vec<String>,
    pub redirs: Vec<String>,
}

impl SimpleCmd {
    pub fn build(input: &str) -> SimpleCmd {
        let (args, redirs) = parse_simple(tokenize(input.trim()));
        SimpleCmd { args, redirs }
    }
}

#[derive(Debug)]
pub enum Token {
    Word(String),
    Op(String),
}

// impl Token {
//     pub fn is_word(&self) -> bool
// }
