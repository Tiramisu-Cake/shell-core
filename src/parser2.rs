use std::str::Chars;

pub fn tokenize(args: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut args = args.chars();

    let mut str_to_push = String::new();

    while let Some(arg) = args.next() {
        if arg == ' ' {
            if !str_to_push.is_empty() {
                result.push(str_to_push);
                str_to_push = String::new();
            }
            continue;
        }
        if arg == '\'' {
            tokenize_single_quotes('\'', &mut args, &mut str_to_push);
            continue;
        } else if arg == '"' {
            tokenize_double_quotes('"', &mut args, &mut str_to_push);
            continue;
        } else if arg == '\\' {
            if let Some(arg) = args.next() {
                str_to_push.push(arg);
            } else {
                str_to_push.push('\\');
            }
            continue;
        }
        str_to_push.push(arg);
    }
    if !str_to_push.is_empty() {
        result.push(str_to_push);
    }
    result
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
