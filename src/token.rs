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

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenType::Plus => write!(f, "+"),
            TokenType::Minus => write!(f, "-"),
            TokenType::Divide => write!(f, "/"),
            TokenType::Multiply => write!(f, "*"),
            TokenType::Power => write!(f, "^"),
            TokenType::Less => write!(f, "<"),
            TokenType::Greater => write!(f, ">"),
            TokenType::Equal => write!(f, "="),
            TokenType::EqualEq => write!(f, "=="),
            TokenType::LessEq => write!(f, "<="),
            TokenType::GreaterEq => write!(f, ">="),
            TokenType::BangEq => write!(f, "!="),
            TokenType::LeftParen => write!(f, "("),
            TokenType::RightParen => write!(f, ")"),
            _ => write!(f, "")
        }
    }
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
