use num_complex::Complex64;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(Complex64),
    Op(char),
    LParen,
    RParen,
    Comma,
    Func(String),
    Var(String),
}
