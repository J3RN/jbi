use crate::Location;
use std::fmt::{self, Display, Formatter};

pub enum Token<'a> {
    Increment(Location<'a>),
    Decrement(Location<'a>),
    Print(Location<'a>),
    MoveRight(Location<'a>),
    MoveLeft(Location<'a>),
    OpenBracket(Location<'a>),
    CloseBracket(Location<'a>),
}

pub enum Error<'a> {
    BadToken {
        trigger: char,
        location: Location<'a>,
    },
}

impl Display for Error<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Error::BadToken {
                trigger,
                location: Location { line, file },
            } => write!(f, "Bad token {} in {} on line {}", trigger, file, line),
        }
    }
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
