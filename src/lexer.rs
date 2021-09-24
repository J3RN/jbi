use crate::Location;
use annotate_snippets::snippet::{Annotation, AnnotationType, Slice, Snippet, SourceAnnotation};

pub enum Token<'a> {
    Increment(Location<'a>),
    Decrement(Location<'a>),
    Print(Location<'a>),
    Read(Location<'a>),
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

impl crate::ErrorOutput for Error<'_> {
    fn to_error(&self) -> Snippet {
        match self {
            Error::BadToken {
                trigger: _,
                location:
                    Location {
                        file,
                        line,
                        lineno,
                        range,
                    },
            } => Snippet {
                title: Some(Annotation {
                    label: Some("Bad token"),
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

pub fn lex<'a>(content: &'a str, file: &'a str) -> Result<Vec<Token<'a>>, Vec<Error<'a>>> {
    let mut res = Vec::<Token>::new();
    let mut errs = Vec::<Error>::new();

    for (lineno, line) in content.lines().enumerate() {
        for (colno, cha) in line.char_indices() {
            match cha {
                '+' => res.push(Token::Increment(loc(file, line, lineno, colno))),
                '-' => res.push(Token::Decrement(loc(file, line, lineno, colno))),
                '.' => res.push(Token::Print(loc(file, line, lineno, colno))),
                ',' => res.push(Token::Read(loc(file, line, lineno, colno))),
                '>' => res.push(Token::MoveRight(loc(file, line, lineno, colno))),
                '<' => res.push(Token::MoveLeft(loc(file, line, lineno, colno))),
                '[' => res.push(Token::OpenBracket(loc(file, line, lineno, colno))),
                ']' => res.push(Token::CloseBracket(loc(file, line, lineno, colno))),
                a => {
                    if !a.is_whitespace() {
                        errs.push(Error::BadToken {
                            trigger: a,
                            location: loc(file, line, lineno, colno),
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

fn loc<'a>(file: &'a str, line: &'a str, lineno: usize, col: usize) -> Location<'a> {
    Location {
        file,
        line,
        lineno: lineno + 1,
        range: (col, col + 1),
    }
}
