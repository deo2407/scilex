#[derive(Debug, PartialEq)]
pub enum TokenType {
    Number(i64),
    Identifier(String),
    String(String),

    Plus, Minus, Multiply, Divide, 
    Less, Greater, Equal,
    LessEq, GreaterEq, EqualEq, BangEq,
    LeftParen, RightParen,

    If, Else, While, For, 
    Fn, Return,
    Set,
    EOF,

    Error,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
}

