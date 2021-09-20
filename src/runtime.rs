use crate::semantic_analyzer::Node;

pub struct State {
    nodes: [u8; 10000],
    head_index: usize,
}

pub trait Run {
    fn eval(&mut self, output: &mut String, node: Node) -> ();
}

impl Default for State {
    fn default() -> State {
        State {
            nodes: [0; 10000],
            head_index: 0,
        }
    }
}

impl Run for State {
    fn eval(&mut self, output: &mut String, node: Node) {
        match node {
            Node::Root { children } => {
                for node in children {
                    self.eval(output, node);
                }
            }
            Node::Loop { children, .. } => {
                while self.nodes[self.head_index] != 0 {
                    for node in &children {
                        self.eval(output, node.clone());
                    }
                }
            }
            Node::Increment { .. } => {
                if self.nodes[self.head_index] < 255 {
                    self.nodes[self.head_index] += 1;
                } else {
                    panic!("Increment past 255!");
                }
            }
            Node::Decrement { .. } => {
                if self.nodes[self.head_index] > 0 {
                    self.nodes[self.head_index] -= 1;
                } else {
                    panic!("Decrement past zero!");
                }
            }
            Node::MoveRight { .. } => {
                if self.head_index < 9999 {
                    self.head_index += 1;
                } else {
                    panic!("Index out of bounds!")
                }
            }
            Node::MoveLeft { .. } => {
                if self.head_index > 0 {
                    self.head_index -= 1;
                } else {
                    panic!("Index out of bounds!")
                }
            }
            Node::Print { .. } => {
                output.push(self.nodes[self.head_index] as char);
            }
        }
    }
}
