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

pub fn eval(state: &mut State, output: &mut String, node: Node) {
    match node {
        Node::Root { children } => {
            for node in children {
                eval(state, output, node);
            }
        }
        Node::Loop { children, .. } => {
            while state.nodes[state.head_index] != 0 {
                for node in &children {
                    eval(state, output, node.clone());
                }
            }
        }
        Node::Increment { .. } => {
            if state.nodes[state.head_index] < 255 {
                state.nodes[state.head_index] += 1;
            } else {
                panic!("Increment past 255!");
            }
        }
        Node::Decrement { .. } => {
            if state.nodes[state.head_index] > 0 {
                state.nodes[state.head_index] -= 1;
            } else {
                panic!("Decrement past zero!");
            }
        }
        Node::MoveRight { .. } => {
            if state.head_index < 9999 {
                state.head_index += 1;
            } else {
                panic!("Index out of bounds!")
            }
        }
        Node::MoveLeft { .. } => {
            if state.head_index > 0 {
                state.head_index -= 1;
            } else {
                panic!("Index out of bounds!")
            }
        }
        Node::Print { .. } => {
            print!("{}", (state.nodes[state.head_index] as char));
        }
    }
}
