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
            Expr::Unary { op, rhs } => write!(f, "{}{}", op, rhs),
            Expr::Binary { lhs, op, rhs } => write!(f, "({} {} {})", op, lhs, rhs),
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

    pub fn parse_expr(&mut self) -> Result<Expr, String> {
        self.parse_expr_bp(0.0)
    }

    fn parse_expr_bp(&mut self, min_bp: f32) -> Result<Expr, String> {
        let token = self.advance().clone();

        let mut lhs = match &token.token_type {
            TokenType::Number(n) => Expr::Number(*n),
            TokenType::LeftParen => {
                let expr = self.parse_expr_bp(0.0)?;
                self.consume(TokenType::RightParen, "Expect ')' after expression")?;
                Expr::Grouping(Box::new(expr))
            },
            TokenType::Minus => {
                let rhs = self.parse_expr_bp(9.0)?;
                Expr::Unary { op: token.token_type.clone(), rhs:  Box::new(rhs)}
            }
            t => return Err(format!("[line {}] Unexpected token '{}' in expression", token.line, t))
        };

        loop {
            let op = match self.match_operator() {
                Some(op) => op,
                None => match &self.peek().token_type {
                    TokenType::EOF | TokenType::RightParen => break,
                    _ => return Err(format!(
                        "[line {}] Unexpected token '{}' in expression",
                        self.peek().line,
                        self.peek().lexeme,
                    )),
                },
            };

            if let Some((l_bp, r_bp)) = Self::infix_binding_power(&op) {
                if l_bp < min_bp {
                    break;
                }

                self.advance(); // consume operator
                let rhs = self.parse_expr_bp(r_bp)?;
                lhs = Expr::Binary {
                    lhs: Box::new(lhs),
                    op: op.clone(),
                    rhs: Box::new(rhs),
                };
                continue;
            };
            break;
        }

        Ok(lhs)
    }

    fn infix_binding_power(op: &TokenType) -> Option<(f32, f32)> {
        let res = match op {
            TokenType::Plus | TokenType::Minus => (1.0, 1.1),
            TokenType::Multiply | TokenType::Divide => (2.0, 2.1),
            _ => return None,
        };
        Some(res)
    }

    fn match_operator(&mut self) -> Option<TokenType> {
        match self.peek().token_type {
            TokenType::Plus
            | TokenType::Minus
            | TokenType::Multiply
            | TokenType::Divide => {
                let tok = self.peek().token_type.clone();
                Some(tok)
            }
            _ => None,
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    #[test]
    fn display_number() {
        let expr = Expr::Number(53); 
        assert_eq!(expr.to_string(), "53");
    }

    #[test]
    fn display_unary() {
        let expr = Expr::Unary { 
            op: TokenType::Minus, 
            rhs: Box::new(Expr::Number(43)),
        };
        assert_eq!(expr.to_string(), "-43");
    }

    #[test]
    fn display_binary() {
        let expr = Expr::Binary { 
            lhs: Box::new(Expr::Number(72)),
            op: TokenType::Plus, 
            rhs: Box::new(Expr::Number(43)),
        };
        assert_eq!(expr.to_string(), "(+ 72 43)");
    }

    #[test]
    fn display_nested() {
        // (1 + 2) * -3
        // ( * (group (+ 1 2)) -3)
        let expr = Expr::Binary {
            lhs: Box::new(Expr::Grouping(Box::new(Expr::Binary {
                lhs: Box::new(Expr::Number(1)),
                op: TokenType::Plus,
                rhs: Box::new(Expr::Number(2)),
            }))),
            op: TokenType::Multiply,
            rhs: Box::new(Expr::Unary {
                op: TokenType::Minus,
                rhs: Box::new(Expr::Number(3)),
            }),
        };

        assert_eq!(expr.to_string(), "(* (group (+ 1 2)) -3)");
    }

    #[test]
    fn parse_expr_add() {
        let tokens = Lexer::lex_all("1 + 2".to_string());

        let mut p = Parser::new(tokens); 
        let expr = p.parse_expr().unwrap();
        assert_eq!(expr.to_string(), "(+ 1 2)") 
    }

    #[test]
    fn parse_expr_mul() {
        let tokens = Lexer::lex_all("41 * 2".to_string());

        let mut p = Parser::new(tokens); 
        let expr = p.parse_expr().unwrap();
        assert_eq!(expr.to_string(), "(* 41 2)") 
    }

    #[test]
    fn parse_expr_mul_error() {
        let tokens = Lexer::lex_all("* 41".to_string());

        let mut p = Parser::new(tokens); 
        let expr = p.parse_expr();

        assert!(expr.is_err());
        assert!(expr.unwrap_err().contains("Unexpected token"));
    }

    #[test]
    fn parse_expr_unary() {
        let tokens = Lexer::lex_all("- 41".to_string());

        let mut p = Parser::new(tokens); 
        let expr = p.parse_expr().unwrap();

        assert_eq!(expr.to_string(), "-41");
    }

    #[test]
    fn parse_expr_unary_nested() {
        let tokens = Lexer::lex_all("- (31 + 12 + 13)".to_string());

        let mut p = Parser::new(tokens); 
        let expr = p.parse_expr().unwrap();

        assert_eq!(expr.to_string(), "-(group (+ (+ 31 12) 13))");
    }

    #[test]
    fn parse_expr_unary_nested_bracket_err() {
        let tokens = Lexer::lex_all("- (31 + 12 + 13".to_string());

        let mut p = Parser::new(tokens); 
        let expr = p.parse_expr();

        assert!(expr.is_err());
        assert!(expr.unwrap_err().contains("Expect ')'"));
    }

    #[test]
    fn parse_precedence() {
        let tokens = Lexer::lex_all("1 + 2 * 3".to_string());

        let mut p = Parser::new(tokens); 
        let expr = p.parse_expr().unwrap();

        assert_eq!(expr.to_string(), "(+ 1 (* 2 3))")
    }

    #[test]
    fn parse_parentheses() {
        let tokens = Lexer::lex_all("(1 + 2) * 3".to_string());

        let mut p = Parser::new(tokens); 
        let expr = p.parse_expr().unwrap();

        assert_eq!(expr.to_string(), "(* (group (+ 1 2)) 3)")
    }
}

