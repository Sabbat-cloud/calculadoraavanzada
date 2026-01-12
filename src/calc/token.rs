#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(f64),
    Op(char),
    LParen,
    RParen,
    Comma,
    Func(String),
    Var(String),
}

