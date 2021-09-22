use crate::lexer::Token;
use crate::Location;

#[derive(Debug)]
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

#[derive(Debug)]
pub enum Error<'a> {
    UnclosedBracketTmp,
    UnclosedBracket(&'a Location<'a>),
    ExtraneousClose(&'a Location<'a>),
}

pub fn analyze<'a>(toks: &'a Vec<Token<'a>>) -> Result<Node<'a>, Vec<Error<'a>>> {
    let mut tok_iter = toks.iter();

    match parse_exp(&mut tok_iter) {
        Ok(children) => Ok(Node::Root { children: children }),
        Err(errs) => Err(errs),
    }
}

pub fn parse_exp<'a>(
    toks: &mut std::slice::Iter<'a, Token<'a>>,
) -> Result<Vec<Node<'a>>, Vec<Error<'a>>> {
    let mut children = Vec::new();
    let mut errs = Vec::new();

    loop {
        match toks.next() {
            Some(Token::OpenBracket(loc)) => match parse_loop(toks) {
                Ok(loop_children) => children.push(Node::Loop {
                    loc: loc,
                    children: loop_children,
                }),
                Err(loop_errs) => {
                    for err in loop_errs {
                        if let Error::UnclosedBracketTmp = err {
                            errs.push(Error::UnclosedBracket(loc))
                        } else {
                            errs.push(err)
                        }
                    }
                }
            },
            Some(Token::CloseBracket(loc)) => errs.push(Error::ExtraneousClose(loc)),
            Some(Token::Increment(loc)) => children.push(Node::Increment { loc: loc }),
            Some(Token::Decrement(loc)) => children.push(Node::Decrement { loc: loc }),
            Some(Token::MoveRight(loc)) => children.push(Node::MoveRight { loc: loc }),
            Some(Token::MoveLeft(loc)) => children.push(Node::MoveLeft { loc: loc }),
            Some(Token::Print(loc)) => children.push(Node::Print { loc: loc }),
            None => break,
        }
    }

    if errs.is_empty() {
        Ok(children)
    } else {
        Err(errs)
    }
}

pub fn parse_loop<'a>(
    toks: &mut std::slice::Iter<'a, Token>,
) -> Result<Vec<Node<'a>>, Vec<Error<'a>>> {
    let mut children = Vec::new();
    let mut errs = Vec::new();

    loop {
        match toks.next() {
            Some(Token::CloseBracket(_)) => break,
            Some(Token::OpenBracket(loc)) => match parse_loop(toks) {
                Ok(loop_children) => children.push(Node::Loop {
                    loc: loc,
                    children: loop_children,
                }),
                Err(loop_errs) => errs.extend(loop_errs),
            },
            Some(Token::Increment(loc)) => children.push(Node::Increment { loc: loc }),
            Some(Token::Decrement(loc)) => children.push(Node::Decrement { loc: loc }),
            Some(Token::MoveRight(loc)) => children.push(Node::MoveRight { loc: loc }),
            Some(Token::MoveLeft(loc)) => children.push(Node::MoveLeft { loc: loc }),
            Some(Token::Print(loc)) => children.push(Node::Print { loc: loc }),
            None => {
                errs.push(Error::UnclosedBracketTmp);
                break;
            }
        }
    }

    if errs.is_empty() {
        Ok(children)
    } else {
        Err(errs)
    }
}
