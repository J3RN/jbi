use crate::semantic_analyzer::Node;

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

pub enum Error {
    NodeOverflow,
    NodeUnderflow,
    HeadOverflow,
    HeadUnderflow,
}

pub fn eval(state: &mut State, node: &Node) -> Result<(), Error> {
    match node {
        Node::Root { children } => {
            for child in children {
                if let Err(e) = eval(state, child) {
                    return Err(e);
                }
            }
            Ok(())
        }
        Node::Loop { children, .. } => {
            while state.nodes[state.head_index] != 0 {
                for child in children {
                    if let Err(e) = eval(state, child) {
                        return Err(e);
                    }
                }
            }
            Ok(())
        }
        Node::Increment { .. } => {
            if state.nodes[state.head_index] < 255 {
                state.nodes[state.head_index] += 1;
                Ok(())
            } else {
                eprintln!("Increment node {} past 255!", state.head_index + 1);
                Err(Error::NodeOverflow)
            }
        }
        Node::Decrement { .. } => {
            if state.nodes[state.head_index] > 0 {
                state.nodes[state.head_index] -= 1;
                Ok(())
            } else {
                eprintln!("Decrement node {} past zero!", state.head_index + 1);
                Err(Error::NodeUnderflow)
            }
        }
        Node::MoveRight { .. } => {
            if state.head_index < 9999 {
                state.head_index += 1;
                Ok(())
            } else {
                eprintln!("Tried to move head past last node!");
                Err(Error::HeadOverflow)
            }
        }
        Node::MoveLeft { .. } => {
            if state.head_index > 0 {
                state.head_index -= 1;
                Ok(())
            } else {
                eprintln!("Tried to move the head before first node!");
                Err(Error::HeadUnderflow)
            }
        }
        Node::Print { .. } => {
            print!("{}", (state.nodes[state.head_index] as char));
            Ok(())
        }
    }
}
