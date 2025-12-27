pub mod cmd;
pub mod parser;
pub mod redirect;
pub mod structs;

use crate::cmd::run::*;
use crate::parser::*;
use crate::structs::ShellState;
use rustyline::config::Configurer;
use rustyline::error::ReadlineError;
use rustyline::history::History;
use std::env;

// current main with rustyline
fn main() {
    let mut state = ShellState::new();
    let _ = state.editor.set_history_ignore_dups(false);

    if let Ok(hist_path) = env::var("HISTFILE") {
        let _ = state.editor.load_history(&hist_path);
        state.history = state.editor.history().len();
    }

    loop {
        let line = state.editor.readline("$ ");
        match line {
            Ok(input) => {
                let trimmed = input.trim();
                if !trimmed.is_empty() {
                    match state.editor.add_history_entry(trimmed) {
                        Ok(_) => (),
                        Err(e) => eprintln!("Couldn't write history: {e}"),
                    }
                }

                let config = parse_pipeline(&tokenize(input.trim()));
                if config.len() <= 1 {
                    run_simplecmd(&mut state, &config[0]);
                    continue;
                }

                let mut it = config.iter();
                let cmd1 = it.next().unwrap();
                let cmd2 = it.next();
                run_pipeline(&mut state, &mut it, cmd1, cmd2, None, Vec::new());
            }
            Err(ReadlineError::Interrupted) => {
                println!("^C");
                continue;
            }
            Err(ReadlineError::Eof) => {
                println!("^D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}
