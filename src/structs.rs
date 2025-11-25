use rustyline::completion::Completer;
use rustyline::Completer;
use rustyline::Context;
use rustyline::DefaultEditor;
use rustyline::Helper;
use rustyline::Highlighter;
use rustyline::Hinter;
use rustyline::Result as RlResult;
use rustyline::Validator;

use std::str;

use rustyline::{history::FileHistory, Editor};

use crate::parser::*;

pub struct ShellState {
    pub editor: Editor<MyHelper, FileHistory>,
    pub history: usize,
}

impl ShellState {
    pub fn new() -> ShellState {
        let helper = MyHelper::new();
        let mut editor = Editor::<MyHelper, FileHistory>::new().expect("failed to create editor");
        editor.set_helper(Some(helper));

        ShellState { editor, history: 0 }
    }
}

#[derive(Helper, Hinter, Highlighter, Validator)]
pub struct MyHelper {
    builtin_cmds: Vec<&'static str>,
}
impl MyHelper {
    pub fn new() -> MyHelper {
        MyHelper {
            builtin_cmds: vec!["cd", "echo", "exit", "type", "pwd", "history"],
        }
    }
}
impl Completer for MyHelper {
    type Candidate = &'static str;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &Context<'_>,
    ) -> RlResult<(usize, Vec<Self::Candidate>)> {
        // Берём всё до курсора
        let before_cursor = &line[..pos];

        // Очень простой вариант: считаем, что дополняем первое слово до первого пробела.
        // Смотрим на "текущий токен" слева от курсора.
        let start = before_cursor
            .rfind(char::is_whitespace) // ищем последний пробел/таб и т.п.
            .map(|idx| idx + 1) // начинаем после него
            .unwrap_or(0); // если пробелов нет — с начала строки

        let prefix = &before_cursor[start..];

        // Фильтруем встроенные команды по префиксу
        let mut matches = Vec::new();
        for &cmd in &self.builtin_cmds {
            if cmd.starts_with(prefix) {
                matches.push(cmd);
            }
        }

        Ok((start, matches))
    }
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
