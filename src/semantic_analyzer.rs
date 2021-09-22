use crate::lexer::Token;
use crate::Location;
use std::fmt::{self, Display, Formatter};

pub enum Node<'a> {
    Root {
        children: Vec<Node<'a>>,
    },
    Loop {
        loc: &'a Location<'a>,
        children: Vec<Node<'a>>,
    },
    Increment {
        loc: &'a Location<'a>,
    },
    Decrement {
        loc: &'a Location<'a>,
    },
    MoveRight {
        loc: &'a Location<'a>,
    },
    MoveLeft {
        loc: &'a Location<'a>,
    },
    Print {
        loc: &'a Location<'a>,
    },
}

pub enum Error<'a> {
    UnclosedBracket(&'a Location<'a>),
    ExtraneousClose(&'a Location<'a>),
}

impl Display for Error<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Error::UnclosedBracket(Location { file, line, col }) => {
                write!(
                    f,
                    "Unclosed '[' in {} on line {} at column {}",
                    file, line, col
                )
            }
            Error::ExtraneousClose(Location { file, line, col }) => {
                write!(
                    f,
                    "Extraneous ']' in {} on line {} at column {}",
                    file, line, col
                )
            }
        }
    }
}

pub fn analyze<'a>(toks: &'a Vec<Token<'a>>) -> Result<Node<'a>, Vec<Error<'a>>> {
    let mut tok_iter = toks.iter().peekable();

    match parse_exp(&mut tok_iter) {
        Ok(children) => {
            if let Some(Token::CloseBracket(loc)) = tok_iter.peek() {
                Err(vec![Error::ExtraneousClose(loc)])
            } else {
                Ok(Node::Root { children: children })
            }
        }
        Err(errs) => Err(errs),
    }
}

pub fn parse_exp<'a>(
    toks: &mut std::iter::Peekable<std::slice::Iter<'a, Token<'a>>>,
) -> Result<Vec<Node<'a>>, Vec<Error<'a>>> {
    let mut children = Vec::new();
    let mut errs = Vec::new();

    loop {
        match toks.peek() {
            Some(Token::OpenBracket(loc)) => {
                toks.next();

                match parse_exp(toks) {
                    Ok(loop_children) => match toks.peek() {
                        Some(Token::CloseBracket(_)) => {
                            children.push(Node::Loop {
                                loc: loc,
                                children: loop_children,
                            });
                        }
                        _ => {
                            errs.push(Error::UnclosedBracket(loc));
                            break;
                        }
                    },
                    Err(loop_errs) => {
                        errs.extend(loop_errs);
                        break;
                    }
                }
            }
            Some(Token::CloseBracket(_)) => break,
            Some(Token::Increment(loc)) => children.push(Node::Increment { loc: loc }),
            Some(Token::Decrement(loc)) => children.push(Node::Decrement { loc: loc }),
            Some(Token::MoveRight(loc)) => children.push(Node::MoveRight { loc: loc }),
            Some(Token::MoveLeft(loc)) => children.push(Node::MoveLeft { loc: loc }),
            Some(Token::Print(loc)) => children.push(Node::Print { loc: loc }),
            None => break,
        }
        toks.next();
    }

    if errs.is_empty() {
        Ok(children)
    } else {
        Err(errs)
    }
}
