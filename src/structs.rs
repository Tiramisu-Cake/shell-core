use std::str;

use crate::parser::*;

pub struct ShellState {
    pub history: Vec<String>,
}

impl ShellState {
    pub fn new() -> ShellState {
        ShellState {
            history: Vec::new(),
        }
    }
}

enum ShellEffect {
    AddHistory,
}

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
    Simple(SimpleCmd),
    Pipeline,
    Sequence,
    Group,
}

// pub enum Stream {
//     Stdout,
//     Stderr,
// }

#[derive(Debug)]
pub struct TargetFile {
    pub path: String,
    pub append: bool,
}

#[derive(Debug)]
pub enum StreamTarget {
    Terminal,
    File(TargetFile),
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

#[derive(Debug)]
pub struct SimpleCmd {
    pub args: Vec<String>,
    pub stdout: StreamTarget,
    pub stderr: StreamTarget,
}

impl SimpleCmd {
    pub fn build(input: &str) -> SimpleCmd {
        parse_simple(&tokenize(input))
    }
}

#[derive(Debug)]
pub enum Token {
    Word(String),
    Op(OpKind),
}

#[derive(Debug)]
pub enum OpKind {
    RedirOutTruncate,
    RedirOutAppend,
    RedirErrTruncate,
    RedirErrAppend,
    Pipeline,
}
// impl Token {
//     pub fn is_word(&self) -> bool
// }
