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

// pub enum Stream {
//     Stdout,
//     Stderr,
// }

pub enum StreamTarget {
    Terminal,
    File(String),
}

// pub struct Redirect {
//     pub redir_type: Stream,
//     pub redir_target: StreamTarget,
// }
//
// impl Default for Redirect {
//     fn default() -> Self {
//         Redirect {
//             redir_type: Stream::Stdout,
//             redir_target: StreamTarget::Terminal,
//         }
//     }
// }
pub struct SimpleCmd {
    pub args: Vec<String>,
    pub stdout: StreamTarget,
    pub stderr: StreamTarget,
}

impl SimpleCmd {
    pub fn build(input: &str) -> SimpleCmd {
        let (args, stdout, stderr) = parse_simple(tokenize(input.trim()));
        SimpleCmd {
            args,
            stdout,
            stderr,
        }
    }
}

#[derive(Debug)]
pub enum Token {
    Word(String),
    Op(OpKind),
}

#[derive(Debug)]
pub enum OpKind {
    RedirToFile,
    RedirErr,
}
// impl Token {
//     pub fn is_word(&self) -> bool
// }
