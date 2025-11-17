use crate::structs::Token;
use std::str::Chars;

const OPERATORS: [&str; 2] = ["1>", ">"];

pub fn tokenize(args: &str) -> Vec<Token> {
    let mut result = Vec::new();
    let mut args = args.chars();

    let mut str_to_push = String::new();

    while let Some(arg) = args.next() {
        match arg {
            ' ' => {
                if !str_to_push.is_empty() {
                    if is_operator(&str_to_push) {
                        match str_to_push.as_str() {
                            "1>" | ">" => result.push(Token::Op("redir1".to_string())),
                            _ => (),
                        }
                    } else {
                        result.push(Token::Word(str_to_push));
                    }
                }
                str_to_push = String::new();
            }
            '\'' => tokenize_single_quotes('\'', &mut args, &mut str_to_push),
            '"' => tokenize_double_quotes('"', &mut args, &mut str_to_push),
            '\\' => {
                if let Some(arg) = args.next() {
                    str_to_push.push(arg);
                } else {
                    str_to_push.push('\\');
                }
            }
            _ => str_to_push.push(arg),
        }
    }
    if !str_to_push.is_empty() {
        result.push(Token::Word(str_to_push));
    }
    result
}

#[test]
fn test_tokenize() {
    let input = "echo hello > file.txt";
    let tokens = tokenize(&input);
    println!("{:?}", tokens);
}

fn is_operator(input: &str) -> bool {
    OPERATORS.contains(&input)
}

fn tokenize_single_quotes(quote: char, args: &mut Chars<'_>, str_to_push: &mut String) {
    while let Some(arg) = args.next() {
        if arg == quote {
            break;
        } else {
            str_to_push.push(arg);
        }
    }
}

fn tokenize_double_quotes(quote: char, args: &mut Chars<'_>, str_to_push: &mut String) {
    while let Some(arg) = args.next() {
        if arg == quote {
            break;
        } else if arg == '\\' {
            if let Some(arg) = args.next() {
                if ['\"', '$', '\\', '`'].contains(&arg) {
                    str_to_push.push(arg);
                } else {
                    str_to_push.push('\\');
                    str_to_push.push(arg);
                }
            } else {
                str_to_push.push('\\');
            }
        } else {
            str_to_push.push(arg);
        }
    }
}

pub fn parse_simple(cmd: Vec<Token>) -> (Vec<String>, Vec<String>) {
    let mut args = Vec::new();
    let mut redirs = Vec::new();
    let mut it = cmd.iter();

    while let Some(token) = it.next() {
        match token {
            Token::Word(s) => args.push(s.clone()),
            Token::Op(_) => {
                if let Some(token) = it.next() {
                    match token {
                        Token::Word(target) => redirs.push(target.clone()),
                        Token::Op(_) => (),
                    }
                }
            }
        }
    }
    (args, redirs)
}
