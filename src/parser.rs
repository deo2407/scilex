use std::fmt;

use crate::lexer::Lexer;
use crate::token::*;

#[derive(Debug)]
enum Expr {
    Number(i64),
    Unary { op: TokenType, rhs: Box<Expr> },
    Binary { lhs: Box<Expr>, op: TokenType, rhs: Box<Expr> },
    Grouping(Box<Expr>),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Number(n) => write!(f, "{}", n),
            Expr::Unary { op, rhs } => write!(f, "{:?} {}", op, rhs),
            Expr::Binary { lhs, op, rhs } => write!(f, "({:?} {} {})", op, lhs, rhs),
            Expr::Grouping(expr) => write!(f, "(group {})", expr)
        }
    }
}

struct Parser {
    pub tokens: Vec<Token>,
    pub current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
        }
    }

    pub fn parse_expr(&mut self, min_bp: f32) -> Expr {
        let token = self.advance();
        let mut lhs = match &token.token_type {
            TokenType::Number(n) => Expr::Number(*n),
            TokenType::LeftParen => {
                let expr = self.parse_expr(0.0);
                self.consume(TokenType::RightParen, "Expect ')' after expression").unwrap();
                Expr::Grouping(Box::new(expr))
            },
            TokenType::Minus => {
                let rhs = self.parse_expr(9.0);
                Expr::Unary { op: TokenType::Minus, rhs:  Box::new(rhs)}
            }
            t => panic!("unexpected token {:?}", t)
        };

        loop {
            let op = match self.peek().token_type.clone() {
                TokenType::EOF => break,
                op => op
            };

            if let Some((l_bp, r_bp)) = Self::infix_binding_power(&op) {
                if l_bp < min_bp {
                    break;
                }

                self.advance(); // consume operator
                let rhs = self.parse_expr(r_bp);
                lhs = Expr::Binary {
                    lhs: Box::new(lhs),
                    op,
                    rhs: Box::new(rhs),
                };
                continue;
            };
            break;
        }

        lhs
    }

    fn infix_binding_power(op: &TokenType) -> Option<(f32, f32)> {
        let res = match op {
            TokenType::Plus | TokenType::Minus => (1.0, 1.1),
            TokenType::Multiply | TokenType::Divide => (2.0, 2.1),
            _ => return None,
        };
        Some(res)
    }

    fn consume(&mut self, token_type: TokenType, error_msg: &str) -> Result<&Token, String> {
        if self.peek().token_type == token_type {
            Ok(self.advance())
        } else {
            Err(format!(
                "[line {}] Error at '{}': {}",
                self.peek().line,
                self.peek().lexeme,
                error_msg,
            ))
        }
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::EOF
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }
}
