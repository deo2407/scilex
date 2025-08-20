use crate::lexer::Lexer;
use crate::token::*;

struct Parser {
    pub current: Token,
    pub previous: Token,
    pub had_error: bool,
}

