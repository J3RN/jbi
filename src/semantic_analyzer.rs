use crate::lexer::Token;

#[derive(Debug, Clone)]
pub enum Node {
    Root { children: Vec<Node> },
    Loop { tok: Token, children: Vec<Node> },
    Increment { tok: Token },
    Decrement { tok: Token },
    MoveRight { tok: Token },
    MoveLeft { tok: Token },
    Print { tok: Token },
}

#[derive(Debug)]
pub struct Error {
    message: String,
}

pub fn analyze(toks: Vec<Token>) -> Result<Node, Vec<Error>> {
    let mut tok_iter = toks.iter();
    let mut errs = Vec::new();
    let mut root = Node::Root {
        children: Vec::new(),
    };

    parse_exp(&mut root, &mut errs, &mut tok_iter);

    if errs.is_empty() {
        Ok(root)
    } else {
        Err(errs)
    }
}

pub fn parse_exp(parent: &mut Node, errs: &mut Vec<Error>, toks: &mut std::slice::Iter<Token>) {
    match parent {
        Node::Root { children } => loop {
            match toks.next() {
                Some(tok @ Token::OpenBracket { .. }) => {
                    let mut new_node = Node::Loop {
                        tok: tok.clone(),
                        children: Vec::new(),
                    };
                    parse_loop(&mut new_node, errs, toks);
                    children.push(new_node);
                }
                Some(Token::CloseBracket { file, line }) => errs.push(Error {
                    message: format!(
                        "Found extraneous close bracket in {} on line {}",
                        file, line
                    ),
                }),
                Some(tok @ Token::Increment { .. }) => {
                    children.push(Node::Increment { tok: tok.clone() })
                }
                Some(tok @ Token::Decrement { .. }) => {
                    children.push(Node::Decrement { tok: tok.clone() })
                }
                Some(tok @ Token::MoveRight { .. }) => {
                    children.push(Node::MoveRight { tok: tok.clone() })
                }
                Some(tok @ Token::MoveLeft { .. }) => {
                    children.push(Node::MoveLeft { tok: tok.clone() })
                }
                Some(tok @ Token::Print { .. }) => children.push(Node::Print { tok: tok.clone() }),
                None => break,
            }
        },
        _ => panic!("Expr parent not root!"),
    }
}

pub fn parse_loop(parent: &mut Node, errs: &mut Vec<Error>, toks: &mut std::slice::Iter<Token>) {
    match parent {
        Node::Loop { tok, children } => loop {
            match toks.next() {
                Some(Token::CloseBracket { .. }) => break,
                Some(Token::OpenBracket { .. }) => {
                    let mut new_node = Node::Loop {
                        tok: tok.clone(),
                        children: Vec::new(),
                    };
                    parse_loop(&mut new_node, errs, toks);
                    children.push(new_node);
                }
                Some(tok @ Token::Increment { .. }) => {
                    children.push(Node::Increment { tok: tok.clone() })
                }
                Some(tok @ Token::Decrement { .. }) => {
                    children.push(Node::Decrement { tok: tok.clone() })
                }
                Some(tok @ Token::MoveRight { .. }) => {
                    children.push(Node::MoveRight { tok: tok.clone() })
                }
                Some(tok @ Token::MoveLeft { .. }) => {
                    children.push(Node::MoveLeft { tok: tok.clone() })
                }
                Some(tok @ Token::Print { .. }) => children.push(Node::Print { tok: tok.clone() }),
                None => match tok {
                    Token::OpenBracket { file, line } => {
                        errs.push(Error {
                            message: format!("Unclosed bracket in {} at {}", file, line),
                        });
                        break;
                    }
                    _ => panic!("Loop parent token not '['!"),
                },
            }
        },
        _ => panic!("Loop parent not loop!"),
    }
}
