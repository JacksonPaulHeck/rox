use crate::*;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum TokenType {
    TokenLeftParen,
    TokenRightParen,
    TokenLeftBrace,
    TokenRightBrace,
    TokenComma,
    TokenDot,
    TokenMinus,
    TokenPlus,
    TokenSemicolon,
    TokenSlash,
    TokenStar,
    TokenBang,
    TokenBangEqual,
    TokenEqual,
    TokenEqualEqual,
    TokenGreater,
    TokenGreaterEqual,
    TokenLess,
    TokenLessEqual,
    TokenIdentifier,
    TokenString,
    TokenNumber,
    TokenAnd,
    TokenClass,
    TokenElse,
    TokenFalse,
    TokenFor,
    TokenFun,
    TokenIf,
    TokenNil,
    TokenOr,
    TokenPrint,
    TokenReturn,
    TokenSuper,
    TokenThis,
    TokenTrue,
    TokenVar,
    TokenWhile,
    TokenError,
    TokenEof,
}

#[derive(Copy, Clone, Debug)]
pub struct Token {
    token_type: TokenType,
    start: [char; 256],
    length: usize,
    line: i64,
}

impl Token {
    pub fn new() -> Token {
        return Token {
            token_type: TokenType::TokenReturn,
            start: ['\0'; 256],
            length: 0,
            line: 0,
        };
    }

    pub fn create(token_type: TokenType, start: [char; 256], length: usize, line: i64) -> Token {
        return Token {
            token_type: token_type,
            start: start,
            length: length,
            line: line,
        };
    }

    pub fn get_type(&self) -> TokenType {
        return self.token_type;
    }

    pub fn get_line(&self) -> i64 {
        return self.line;
    }

    pub fn get_start(&self) -> [char; 256] {
        return self.start;
    }

    pub fn get_length(&self) -> usize {
        return self.length;
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Scanner {
    start: [char; 256],
    tok_beg: usize,
    current: usize,
    line: i64,
}

impl Scanner {
    pub fn new() -> Scanner {
        return Scanner {
            start: ['\0'; 256],
            tok_beg: 0,
            current: 0,
            line: 0,
        };
    }

    pub fn create(source: String) -> Scanner {
        let chars = source.chars().into_iter().collect::<Vec<char>>();
        let len: usize = source.chars().into_iter().collect::<Vec<char>>().len();

        let mut src: [char; 256] = ['\0'; 256];

        for i in 0..len {
            src[i] = chars[i];
        }

        return Scanner {
            start: src,
            tok_beg: 0,
            current: 0,
            line: 0,
        };
    }

    fn create_error_token(&self) -> Token {
        let chars = "Unexpected Character"
            .chars()
            .into_iter()
            .collect::<Vec<char>>();
        let len: usize = "Unexpected Character"
            .chars()
            .into_iter()
            .collect::<Vec<char>>()
            .len();
        let mut src: [char; 256] = ['\0'; 256];

        for i in 0..len {
            src[i] = chars[i];
        }

        return Token::create(
            TokenType::TokenError,
            src,
            len.try_into().unwrap(),
            self.line,
        );
    }

    fn create_token_from_type(&self, token_type: TokenType) -> Token {
        let mut data: [char; 256] = ['\0'; 256];
        let tmp = &self.start[self.tok_beg..self.current];

        for i in 0..tmp.len() {
            data[i] = tmp[i];
        }

        return Token::create(token_type, data, self.current - self.tok_beg, self.line);
    }

    fn check_keyword(
        &self,
        start: i64,
        length: i64,
        rest: String,
        token_type: TokenType,
    ) -> TokenType {
        if self.current - self.tok_beg == (start + length).try_into().unwrap() {
            return token_type;
        }
        return TokenType::TokenIdentifier;
    }

    fn identifier_type(&self) -> TokenType {
        match self.start[self.tok_beg] {
            'a' => return self.check_keyword(1, 2, "nd".to_string(), TokenType::TokenAnd),
            'c' => todo!(),
            'e' => todo!(),
            'f' => todo!(),
            'i' => todo!(),
            'n' => todo!(),
            'o' => todo!(),
            'p' => return self.check_keyword(1, 4, "rint".to_string(), TokenType::TokenPrint),
            'r' => todo!(),
            's' => todo!(),
            't' => todo!(),
            'v' => return self.check_keyword(1, 2, "ar".to_string(), TokenType::TokenVar),
            'w' => todo!(),
            _ => return TokenType::TokenIdentifier,
        }
    }

    fn identifier(&mut self) -> Token {
        while self.is_alpha(self.start[self.current]) || self.is_digit(self.start[self.current]) {
            self.advance();
        }

        return self.create_token_from_type(self.identifier_type());
    }

    fn number(&mut self) -> Token {
        while self.is_digit(self.start[self.current]) {
            self.advance();
        }
        if self.start[self.current] == '.' && self.is_digit(self.start[self.current + 1]) {
            self.advance();
            while self.is_digit(self.start[self.current]) {
                self.advance();
            }
        }
        return self.create_token_from_type(TokenType::TokenNumber);
    }

    fn is_at_end(&self) -> bool {
        return self.start[self.current + 1] == '\0';
    }
    fn is_alpha(&self, c: char) -> bool {
        return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '-';
    }
    fn is_digit(&self, c: char) -> bool {
        return c >= '0' && c <= '9';
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        return self.start[self.current - 1];
    }

    fn skip_whitespace(&mut self) {
        loop {
            let c: char = self.start[self.current];
            match c {
                ' ' | '\r' | '\t' | '\0' => {
                    self.advance();
                    break;
                }
                '\n' => {
                    self.line += 1;
                    self.advance();
                    return;
                }
                '/' => todo!(),
                _ => return,
            }
        }
    }

    fn match_to(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.start[self.current] != expected {
            return false;
        }
        self.current += 1;
        return true;
    }

    pub fn scan_token(&mut self) -> Token {
        self.skip_whitespace();
        self.tok_beg = self.current;

        if self.is_at_end() {
            return self.create_token_from_type(TokenType::TokenEof);
        }
        let c: char = self.advance();
        if self.is_alpha(c) {
            return self.identifier();
        }

        if self.is_digit(c) {
            return self.number();
        }

        match c {
            '(' => todo!(),
            ')' => todo!(),
            '{' => todo!(),
            '}' => todo!(),
            ';' => return self.create_token_from_type(TokenType::TokenSemicolon),
            ',' => todo!(),
            '.' => todo!(),
            '-' => todo!(),
            '+' => return self.create_token_from_type(TokenType::TokenPlus),
            '/' => todo!(),
            '*' => todo!(),
            '!' => todo!(),
            '=' => {
                if self.match_to('=') {
                    return self.create_token_from_type(TokenType::TokenEqualEqual);
                } else {
                    return self.create_token_from_type(TokenType::TokenEqual);
                }
            }
            '<' => todo!(),
            '>' => todo!(),
            '"' => todo!(),
            _ => {
                return self.create_error_token();
            }
        }
    }

    pub fn get_start(&self) -> [char; 256] {
        return self.start;
    }

    pub fn get_line(&self) -> i64 {
        return self.line;
    }

    pub fn get_current(&self) -> usize {
        return self.current;
    }
}
