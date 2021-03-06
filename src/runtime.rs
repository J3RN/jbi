use crate::{parser::Node, Location};
use annotate_snippets::snippet::{Annotation, AnnotationType, Slice, Snippet, SourceAnnotation};
use std::io::{self, Read};

const NODE_LEN: usize = 30_000;

pub struct State {
    nodes: [u8; NODE_LEN],
    head_index: usize,
}

impl Default for State {
    fn default() -> State {
        State {
            nodes: [0; NODE_LEN],
            head_index: 0,
        }
    }
}

pub enum Error<'a> {
    NodeOverflow(usize, &'a Location<'a>),
    NodeUnderflow(usize, &'a Location<'a>),
    HeadOverflow(&'a Location<'a>),
    HeadUnderflow(&'a Location<'a>),
    ReadFailure(&'a Location<'a>),
}

impl crate::ErrorOutput for Error<'_> {
    fn to_error(&self) -> Snippet {
        match self {
            Error::NodeOverflow(
                ..,
                Location {
                    file,
                    line,
                    lineno,
                    range,
                },
            ) => Snippet {
                title: Some(Annotation {
                    label: Some("Node Overflow"),
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
            Error::NodeUnderflow(
                ..,
                Location {
                    file,
                    line,
                    lineno,
                    range,
                },
            ) => Snippet {
                title: Some(Annotation {
                    label: Some("Node Underflow"),
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
            Error::HeadOverflow(Location {
                file,
                line,
                lineno,
                range,
            }) => Snippet {
                title: Some(Annotation {
                    label: Some("Head Overflow"),
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
            Error::HeadUnderflow(Location {
                file,
                line,
                lineno,
                range,
            }) => Snippet {
                title: Some(Annotation {
                    label: Some("Head Underflow"),
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
            Error::ReadFailure(Location {
                file,
                line,
                lineno,
                range,
            }) => Snippet {
                title: Some(Annotation {
                    label: Some("Read Error"),
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

pub fn eval<'a>(state: &mut State, node: &'a Node) -> Result<(), Error<'a>> {
    match node {
        Node::Root { children } => {
            for child in children {
                eval(state, child)?;
            }
            Ok(())
        }
        Node::Loop { children, .. } => {
            while state.nodes[state.head_index] != 0 {
                for child in children {
                    eval(state, child)?;
                }
            }
            Ok(())
        }
        Node::Increment { loc } => {
            if state.nodes[state.head_index] < 255 {
                state.nodes[state.head_index] += 1;
                Ok(())
            } else {
                Err(Error::NodeOverflow(state.head_index + 1, loc))
            }
        }
        Node::Decrement { loc } => {
            if state.nodes[state.head_index] > 0 {
                state.nodes[state.head_index] -= 1;
                Ok(())
            } else {
                Err(Error::NodeUnderflow(state.head_index + 1, loc))
            }
        }
        Node::MoveRight { loc } => {
            if state.head_index < NODE_LEN - 1 {
                state.head_index += 1;
                Ok(())
            } else {
                Err(Error::HeadOverflow(loc))
            }
        }
        Node::MoveLeft { loc } => {
            if state.head_index > 0 {
                state.head_index -= 1;
                Ok(())
            } else {
                Err(Error::HeadUnderflow(loc))
            }
        }
        Node::Print { .. } => {
            print!("{}", (state.nodes[state.head_index] as char));
            Ok(())
        }
        Node::Read { loc } => {
            match io::stdin().read(&mut state.nodes[state.head_index..state.head_index + 1]) {
                Ok(_) => Ok(()),
                Err(_) => Err(Error::ReadFailure(loc)),
            }
        }
    }
}
