use crate::lexer::Token;
use crate::Location;
use annotate_snippets::snippet::{Annotation, AnnotationType, Slice, Snippet, SourceAnnotation};

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
    Read {
        loc: &'a Location<'a>,
    },
}

pub enum Error<'a> {
    UnclosedBracket(&'a Location<'a>),
    ExtraneousClose(&'a Location<'a>),
}

impl crate::ErrorOutput for Error<'_> {
    fn to_error(&self) -> Snippet {
        match self {
            Error::UnclosedBracket(Location {
                file,
                line,
                lineno,
                range,
            }) => Snippet {
                title: Some(Annotation {
                    label: Some("Unclosed Bracket"),
                    id: None,
                    annotation_type: AnnotationType::Error,
                }),
                footer: vec![],
                slices: vec![Slice {
                    source: line,
                    line_start: *lineno,
                    origin: Some(file),
                    fold: false,
                    annotations: vec![SourceAnnotation {
                        label: "",
                        annotation_type: AnnotationType::Error,
                        range: *range,
                    }],
                }],
                opt: Default::default(),
            },
            Error::ExtraneousClose(Location {
                file,
                line,
                lineno,
                range,
            }) => Snippet {
                title: Some(Annotation {
                    label: Some("Extraneous Close Bracket"),
                    id: None,
                    annotation_type: AnnotationType::Error,
                }),
                footer: vec![],
                slices: vec![Slice {
                    source: line,
                    line_start: *lineno,
                    origin: Some(file),
                    fold: false,
                    annotations: vec![SourceAnnotation {
                        label: "",
                        annotation_type: AnnotationType::Error,
                        range: *range,
                    }],
                }],
                opt: Default::default(),
            },
        }
    }
}

pub fn parse<'a>(toks: &'a [Token<'a>]) -> Result<Node<'a>, Vec<Error<'a>>> {
    let mut tok_iter = toks.iter().peekable();

    match parse_exp(&mut tok_iter) {
        Ok(children) => {
            if let Some(Token::CloseBracket(loc)) = tok_iter.peek() {
                Err(vec![Error::ExtraneousClose(loc)])
            } else {
                Ok(Node::Root { children })
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
                                loc,
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
            Some(Token::Increment(loc)) => children.push(Node::Increment { loc }),
            Some(Token::Decrement(loc)) => children.push(Node::Decrement { loc }),
            Some(Token::MoveRight(loc)) => children.push(Node::MoveRight { loc }),
            Some(Token::MoveLeft(loc)) => children.push(Node::MoveLeft { loc }),
            Some(Token::Print(loc)) => children.push(Node::Print { loc }),
            Some(Token::Read(loc)) => children.push(Node::Read { loc }),
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
