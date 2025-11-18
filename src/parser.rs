use crate::structs::{OpKind, SimpleCmd, StreamTarget, TargetFile, Token};
use std::str::Chars;

const OPERATORS: [&str; 7] = ["1>", ">", "1>>", ">>", "2>", "2>>", "|"];

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
                            "1>" | ">" => result.push(Token::Op(OpKind::RedirOutTruncate)),
                            "2>" => result.push(Token::Op(OpKind::RedirErrTruncate)),
                            "1>>" | ">>" => result.push(Token::Op(OpKind::RedirOutAppend)),
                            "2>>" => result.push(Token::Op(OpKind::RedirErrAppend)),
                            "|" => result.push(Token::Op(OpKind::Pipeline)),
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

pub fn parse_pipeline(cmd: &[Token]) -> Vec<SimpleCmd> {
    let mut result = Vec::new();
    for (i, token) in cmd.iter().enumerate() {
        match token {
            Token::Op(OpKind::Pipeline) => {
                result.push(parse_simple(&cmd[0..i]));
                result.append(&mut parse_pipeline(&cmd[i + 1..]));
                break;
            }
            _ => (),
        }
    }
    if result.is_empty() {
        result.push(parse_simple(cmd));
    }
    result
}

#[test]
fn test_pipeline() {
    let input = "cat /var/log/syslog \
  | grep \"ERROR\" \
  | awk '{print $1, $2, $5}' \
  | sort \
  | uniq -c \
  | sort -nr \
  | head -20
";
    let input2 = "ls -1 nonexistent 2>> /tmp/bee/cow.md";
    let tokens = parse_pipeline(&tokenize(&input2));
    println!("{:?}", tokens);
}

pub fn parse_simple(cmd: &[Token]) -> SimpleCmd {
    let mut args = Vec::new();
    let mut stdout = StreamTarget::Terminal;
    let mut stderr = StreamTarget::Terminal;

    let mut it = cmd.iter();

    while let Some(token) = it.next() {
        match token {
            Token::Word(s) => args.push(s.clone()),
            Token::Op(OpKind::RedirOutTruncate) => {
                if let Some(token) = it.next() {
                    redirect(token, &mut stdout, false);
                }
            }
            Token::Op(OpKind::RedirErrTruncate) => {
                if let Some(token) = it.next() {
                    redirect(token, &mut stderr, false);
                }
            }
            Token::Op(OpKind::RedirErrAppend) => {
                if let Some(token) = it.next() {
                    redirect(token, &mut stderr, true);
                }
            }
            Token::Op(OpKind::RedirOutAppend) => {
                if let Some(token) = it.next() {
                    redirect(token, &mut stdout, true);
                }
            }
            Token::Op(OpKind::Pipeline) => (),
        }
    }
    SimpleCmd {
        args,
        stdout,
        stderr,
    }
}

fn redirect(token: &Token, std: &mut StreamTarget, append: bool) {
    match token {
        Token::Word(target) => {
            let target = TargetFile {
                path: target.to_owned(),
                append,
            };
            *std = StreamTarget::File(target);
        }
        Token::Op(_) => (),
    }
}
