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
                location: Location { file, line, col },
            } => write!(
                f,
                "Bad token {} in {} on line {} at column {}",
                trigger, file, line, col
            ),
        }
    }
}

pub fn lex(content: String, file: &str) -> Result<Vec<Token>, Vec<Error>> {
    let mut res = Vec::<Token>::new();
    let mut errs = Vec::<Error>::new();

    for (lineno, content) in content.lines().enumerate() {
        let line = lineno + 1;

        for (colno, cha) in content.chars().enumerate() {
            let col = colno + 1;

            match cha {
                '+' => res.push(Token::Increment(Location { file, line, col })),
                '-' => res.push(Token::Decrement(Location { file, line, col })),
                '.' => res.push(Token::Print(Location { file, line, col })),
                '>' => res.push(Token::MoveRight(Location { file, line, col })),
                '<' => res.push(Token::MoveLeft(Location { file, line, col })),
                '[' => res.push(Token::OpenBracket(Location { file, line, col })),
                ']' => res.push(Token::CloseBracket(Location { file, line, col })),
                a => {
                    if !a.is_whitespace() {
                        errs.push(Error::BadToken {
                            trigger: a,
                            location: Location { file, line, col },
                        })
                    }
                }
            }
        }
    }

    if errs.is_empty() {
        Ok(res)
    } else {
        Err(errs)
    }
}
