#[derive(Debug, Clone)]
pub enum Token {
    Increment { line: i32, file: String },
    Decrement { line: i32, file: String },
    Print { line: i32, file: String },
    MoveRight { line: i32, file: String },
    MoveLeft { line: i32, file: String },
    OpenBracket { line: i32, file: String },
    CloseBracket { line: i32, file: String },
}

#[derive(Debug)]
pub enum Error {
    BadTokens {
        trigger: char,
        line: i32,
        file: String,
    },
}

pub fn lex(content: String, file: String) -> Result<Vec<Token>, Vec<Error>> {
    let mut res = Vec::<Token>::new();
    let mut errs = Vec::<Error>::new();
    let mut line = 1;

    for cha in content.chars() {
        match cha {
            '+' => res.push(Token::Increment {
                line: line,
                file: file.clone(),
            }),
            '-' => res.push(Token::Decrement {
                line: line,
                file: file.clone(),
            }),
            '.' => res.push(Token::Print {
                line: line,
                file: file.clone(),
            }),
            '>' => res.push(Token::MoveRight {
                line: line,
                file: file.clone(),
            }),
            '<' => res.push(Token::MoveLeft {
                line: line,
                file: file.clone(),
            }),
            '[' => res.push(Token::OpenBracket {
                line: line,
                file: file.clone(),
            }),
            ']' => res.push(Token::CloseBracket {
                line: line,
                file: file.clone(),
            }),
            '\n' => line = line + 1,
            a => {
                if !a.is_whitespace() {
                    errs.push(Error::BadTokens {
                        trigger: a,
                        line: line,
                        file: file.clone(),
                    })
                }
            }
        };
    }

    if errs.is_empty() {
        Ok(res)
    } else {
        Err(errs)
    }
}
