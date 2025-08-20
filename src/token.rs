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

