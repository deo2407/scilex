use crate::token::{TokenType, Token};

pub struct Lexer {
    chars: Vec<char>,
    start: usize,      // start of current token
    current: usize,    // current position in source
    line: usize,
}

impl Lexer {
    fn new(source: &str) -> Self {
        Self {
            chars: source.chars().collect(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.chars.len()
    }

    fn advance(&mut self) -> char {
        let ch = self.chars[self.current]; 
        self.current += 1;
        ch
    }

    fn peek(&self) -> Option<char> {
        self.chars.get(self.current).copied()
    }

    fn peek_next(&self) -> Option<char> {
        self.chars.get(self.current + 1).copied()
    }

    fn get_lexeme(&self) -> String {
        self.chars[self.current..self.start].iter().collect()
    }

    fn is_alpha(c: char) -> bool {
        (c >= 'a' && c <= 'z') | (c >= 'A' && c <= 'Z')
    }

    fn is_digit(c: char) -> bool {
        c >= '0' && c <= '9'
    }

    fn match_char(&mut self, expect: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.chars[self.current] != expect {
            return false;
        }
        self.current += 1;

        true
    }

    fn make_token(&self, token_type: TokenType) -> Token {
        Token {
            token_type,
            lexeme: self.get_lexeme(),
            line: self.line,
        }
    }

    fn error_token(&self, message: String) -> Token {
        Token {
            token_type: TokenType::Error,
            lexeme: message,
            line: self.line
        }
    }

    fn skip_whitespace(&mut self) {
        loop {
            let c = self.peek();
            match c {
                Some(' ') | Some('\r') | Some('\t') => {
                    self.advance();
                },
                Some('\n') => {
                    self.line += 1;
                    self.advance();
                },
                Some('/') => {
                    // consume comments
                    if self.peek_next() == Some('/') {
                        while let Some(ch) = self.peek() {
                            if ch == '\n' { break; }
                            self.advance();
                        }
                    } else {
                        break;
                    }
                }
                _ => break,
            }
        } 
    }

    fn number(&mut self) -> Token {
        while Self::is_digit(self.peek().unwrap_or(' ')) { self.advance(); }

        let num_str = self.chars[self.start..self.current]
            .iter()
            .collect::<String>();
        let num: i64 = num_str.parse().unwrap();

        self.make_token(TokenType::Number(num))
    }

    fn scan_token(&mut self) -> Token {
        self.skip_whitespace(); 
        self.start = self.current;

        if self.is_at_end() {
            return self.make_token(TokenType::EOF);
        }

        let c = self.advance();

        if Self::is_digit(c) {
            self.number();
        }

        match c {
            '(' => self.make_token(TokenType::LeftParen),
            ')' => self.make_token(TokenType::RightParen),
            '+' => self.make_token(TokenType::Plus),
            '-' => self.make_token(TokenType::Minus),
            '*' => self.make_token(TokenType::Multiply),
            '/' => self.make_token(TokenType::Divide),
            _ => self.make_token(TokenType::Error)
        }
    }
}

