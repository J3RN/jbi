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

pub fn eval(state: &mut State, node: &Node) -> bool {
    match node {
        Node::Root { children } => {
            for node in children {
                if !eval(state, node) {
                    return false;
                }
            }
            true
        }
        Node::Loop { children, .. } => {
            while state.nodes[state.head_index] != 0 {
                for node in children {
                    if !eval(state, node) {
                        return false;
                    }
                }
            }
            true
        }
        Node::Increment { .. } => {
            if state.nodes[state.head_index] < 255 {
                state.nodes[state.head_index] += 1;
                true
            } else {
                eprintln!("Increment node {} past 255!", state.head_index + 1);
                false
            }
        }
        Node::Decrement { .. } => {
            if state.nodes[state.head_index] > 0 {
                state.nodes[state.head_index] -= 1;
                true
            } else {
                eprintln!("Decrement node {} past zero!", state.head_index + 1);
                false
            }
        }
        Node::MoveRight { .. } => {
            if state.head_index < 9999 {
                state.head_index += 1;
                true
            } else {
                eprintln!("Tried to move head past last node!");
                false
            }
        }
        Node::MoveLeft { .. } => {
            if state.head_index > 0 {
                state.head_index -= 1;
                true
            } else {
                eprintln!("Tried to move the head before first node!");
                false
            }
        }
        Node::Print { .. } => {
            print!("{}", (state.nodes[state.head_index] as char));
            true
        }
    }
}
