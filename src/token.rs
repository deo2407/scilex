use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Number(i64),
    Identifier(String),
    String(String),

    Plus, Minus, Multiply, Divide, Power,
    Less, Greater, Equal,
    LessEq, GreaterEq, EqualEq, BangEq,
    LeftParen, RightParen,

    If, Else, While, For, 
    Fn, Return,
    Set,
    NIL,
    EOF,

    Error,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.lexeme)
    }
}
