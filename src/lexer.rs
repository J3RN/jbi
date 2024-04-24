use crate::Location;

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

pub fn lex<'a>(content: &'a str, file: &'a str) -> Vec<Token<'a>> {
    let mut res = Vec::<Token>::new();

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
                _ => {} /* All non-command tokens are ignored */
            }
        }
    }

    res
}

fn loc<'a>(file: &'a str, line: &'a str, lineno: usize, col: usize) -> Location<'a> {
    Location {
        file,
        line,
        lineno: lineno + 1,
        range: (col, col + 1),
    }
}
