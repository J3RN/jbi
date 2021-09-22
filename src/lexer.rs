use crate::Location;

#[derive(Debug)]
pub enum Token<'a> {
    Increment(Location<'a>),
    Decrement(Location<'a>),
    Print(Location<'a>),
    MoveRight(Location<'a>),
    MoveLeft(Location<'a>),
    OpenBracket(Location<'a>),
    CloseBracket(Location<'a>),
}

#[derive(Debug)]
pub enum Error<'a> {
    BadToken {
        trigger: char,
        location: Location<'a>,
    },
}

pub fn lex(content: String, file: &str) -> Result<Vec<Token>, Vec<Error>> {
    let mut res = Vec::<Token>::new();
    let mut errs = Vec::<Error>::new();
    let mut line = 1;

    for cha in content.chars() {
        match cha {
            '+' => res.push(Token::Increment(Location {
                line: line,
                file: file,
            })),
            '-' => res.push(Token::Decrement(Location {
                line: line,
                file: file,
            })),
            '.' => res.push(Token::Print(Location {
                line: line,
                file: file,
            })),
            '>' => res.push(Token::MoveRight(Location {
                line: line,
                file: file,
            })),
            '<' => res.push(Token::MoveLeft(Location {
                line: line,
                file: file,
            })),
            '[' => res.push(Token::OpenBracket(Location {
                line: line,
                file: file,
            })),
            ']' => res.push(Token::CloseBracket(Location {
                line: line,
                file: file,
            })),
            '\n' => line = line + 1,
            a => {
                if !a.is_whitespace() {
                    errs.push(Error::BadToken {
                        trigger: a,
                        location: Location { file, line },
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
