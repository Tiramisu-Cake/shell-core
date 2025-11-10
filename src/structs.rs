pub struct Config {
    pub cmd: String,
    pub args: Vec<String>,
}

impl Config {
    pub fn build(input: String) -> Config {
        let mut parts = input.trim().splitn(2, " ");

        let cmd = parts.next().unwrap().to_string();
        let input = parts
            .next()
            .map(|s| Self::parse_args(Self::tokenize(s.trim())));
        let mut args = Vec::new();
        if let Some(input_unpacked) = input {
            args = input_unpacked;
        } else {
        }

        Config { cmd, args }
    }
    pub fn tokenize(args: &str) -> Vec<String> {
        let mut result = Vec::new();
        let mut args = args.chars();

        let mut str_to_push = String::new();
        while let Some(arg) = args.next() {
            if arg == ' ' {
                continue;
            }
            if arg == '\'' {
                str_to_push.push('\'');
                let mut enough_quotes = false;
                while let Some(arg) = args.next() {
                    if arg == '\'' {
                        enough_quotes = !enough_quotes;
                    }
                    if arg == ' ' && enough_quotes {
                        break;
                    }
                    str_to_push.push(arg);
                }
                result.push(str_to_push);
                str_to_push = String::new();
            } else if arg == '\"' {
                str_to_push.push('\"');
                let mut enough_quotes = false;
                while let Some(arg) = args.next() {
                    if arg == '\"' {
                        enough_quotes = !enough_quotes;
                    }
                    if arg == ' ' && enough_quotes {
                        break;
                    }
                    str_to_push.push(arg);
                }
                result.push(str_to_push);
                str_to_push = String::new();
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

    fn parse_args(args: Vec<String>) -> Vec<String> {
        let mut new_args: Vec<String> = Vec::new();
        for (i, word) in args.iter().enumerate() {
            if word.starts_with("'") {
                new_args.push(word.replace("'", ""));
            } else if word.contains("\"") {
                new_args.push(word.replace("\"", ""));
            } else {
                new_args.push(word.clone());
            }
        }
        new_args
    }
}
