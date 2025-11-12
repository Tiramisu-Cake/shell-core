use std::mem;
use std::str::Chars;

pub fn tokenize(args: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut args = args.chars();

    let mut str_to_push = String::new();
    while let Some(arg) = args.next() {
        if arg == ' ' {
            continue;
        }
        if arg == '\'' {
            tokenize_quotes('\'', &mut args, &mut result, &mut str_to_push);
        } else if arg == '\"' {
            tokenize_quotes('\"', &mut args, &mut result, &mut str_to_push);
        } else if arg == '\\' {
            println!("I am here!!");
            if let Some(arg) = args.next() {
                str_to_push.push(arg);
            }
        } else {
            str_to_push.push(arg);
            while let Some(arg) = args.next() {
                if arg == ' ' {
                    break;
                }
                str_to_push.push(arg);
            }
            result.push(str_to_push);
            str_to_push = String::new();
        }
    }
    result
}

fn tokenize_quotes(
    quote: char,
    args: &mut Chars<'_>,
    result: &mut Vec<String>,
    str_to_push: &mut String,
) {
    str_to_push.push(quote);
    let mut enough_quotes = false;
    while let Some(arg) = args.next() {
        if arg == quote {
            enough_quotes = !enough_quotes;
        }
        if arg == ' ' && enough_quotes {
            break;
        }
        str_to_push.push(arg);
    }
    result.push(mem::take(str_to_push)); //move str_to_push and replace it with the new one
}

pub fn parse_args(args: Vec<String>) -> Vec<String> {
    let mut new_args: Vec<String> = Vec::new();
    for (i, word) in args.iter().enumerate() {
        if word.starts_with("'") || word.contains("''") {
            new_args.push(word.replace("'", ""));
        } else if word.contains("\"") {
            new_args.push(word.replace("\"", ""));
        } else {
            new_args.push(word.clone());
        }
    }
    new_args
}
