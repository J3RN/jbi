use crate::{semantic_analyzer::Node, Location};
use std::fmt::{self, Display, Formatter};

pub struct State {
    nodes: [u8; 10000],
    head_index: usize,
}

impl Default for State {
    fn default() -> State {
        State {
            nodes: [0; 10000],
            head_index: 0,
        }
    }
}

pub enum Error<'a> {
    NodeOverflow(usize, &'a Location<'a>),
    NodeUnderflow(usize, &'a Location<'a>),
    HeadOverflow(&'a Location<'a>),
    HeadUnderflow(&'a Location<'a>),
}

impl Display for Error<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let loc = match self {
            Error::NodeOverflow(node, loc) => {
                writeln!(f, "Tried to increment node {} past 255!", node)?;
                loc
            }
            Error::NodeUnderflow(node, loc) => {
                writeln!(f, "Tried to decrement node {} past zero!", node)?;
                loc
            }
            Error::HeadOverflow(loc) => {
                writeln!(f, "Tried to move the head past the last node!")?;
                loc
            }
            Error::HeadUnderflow(loc) => {
                writeln!(f, "Tried to move the head before first node!")?;
                loc
            }
        };

        write!(
            f,
            "Offending instruction in {} on line {} at column {}",
            loc.file, loc.line, loc.col
        )
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
            if state.head_index < 9999 {
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
    }
}
