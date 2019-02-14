use std::iter::Peekable;
use std::str::Chars;

use crate::ring::Ring;
use crate::token;
use crate::token::Token;
use crate::token::TokenType;
use crate::token::TokenValue;

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
    line: i32,
    column: i32,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &str) -> Lexer {
        Lexer {
            input: input.chars().peekable(),
            line: 1,
            column: 1,
        }
    }

    pub fn lex(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        loop {
            let tok = self.next_token();
            if tok.token_type == TokenType::End {
                break;
            }
            tokens.push(tok)
        }
        tokens
    }

    fn gen_token(&self, t: TokenType, v: TokenValue) -> Token {
        Token {
            token_type: t,
            value: v,
            line: self.line,
            column: self.column,
            range: v.len() as i32,
        }
    }

    fn read_char(&mut self) -> Option<char> {
        let next = match self.input.next() {
            Some(v) => v,
            None => return None,
        };
        if next == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        Some(next)
    }

    fn peek_char(&mut self) -> Option<&char> {
        self.input.peek()
    }

    // fn peek_char_eq(&mut self, ch: char) -> bool {
    //     match self.peek_char() {
    //         Some(&peek_ch) => peek_ch == ch,
    //         None => false,
    //     }
    // }

    fn peek_char_eq_consume(&mut self, ch: char) -> bool {
        match self.peek_char() {
            Some(&peek_ch) => {
                if peek_ch == ch {
                    self.read_char();
                    true
                } else {
                    false
                }
            }
            None => false,
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(&c) = self.peek_char() {
            if !c.is_whitespace() {
                break;
            }
            self.read_char();
        }
    }

    fn peek_is_letter(&mut self) -> bool {
        match self.peek_char() {
            Some(&ch) => is_letter(ch),
            None => false,
        }
    }

    fn read_string_until(&mut self, until: &str) -> String {
        let mut result = String::new();
        let until_len = until.len();
        let mut recent_chars = Ring::new(until_len);

        while let Some(c) = self.read_char() {
            recent_chars.insert(c as u8);
            if String::from(until) == String::from_utf8(recent_chars.unroll()).unwrap() {
                result.truncate(result.len() - until_len);
                break;
            }
            result.push(c);
        }
        result
    }

    fn read_until_eol(&mut self) -> String {
        let mut line = String::new();
        while let Some(c) = self.read_char() {
            if c == '\n' {
                break;
            }
            line.push(c);
        }
        line
    }

    fn read_symbol(&mut self, first: char) -> Token {
        let mut ident = String::new();
        ident.push(first);

        while self.peek_is_letter() {
            ident.push(self.read_char().unwrap());
        }

        Token {
            token_type: token::lookup_keyword(&ident),
            value: TokenValue::String(ident),
            line: 0,
            column: 0,
            range: 0,
        }
    }

    fn read_number(&mut self, first: char) -> Token {
        let mut number = String::new();
        number.push(first);

        let mut has_decimal = false;
        while let Some(&c) = self.peek_char() {
            if !c.is_numeric() {
                if c == '.' {
                    has_decimal = true
                } else if c == '_' {
                    continue;
                } else {
                    break;
                }
            }

            number.push(self.read_char().unwrap());
        }

        if has_decimal {
            self.gen_token(TokenType::Float, TokenValue::Float(number.parse().unwrap()))
        } else {
            self.gen_token(
                TokenType::Integer,
                TokenValue::Integer(number.parse().unwrap()),
            )
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        match self.read_char() {
            Some('=') => {
                if self.peek_char_eq_consume('=') {
                    self.gen_token(TokenType::Equal, TokenValue::None)
                } else {
                    self.gen_token(TokenType::Assign, TokenValue::None)
                }
            }
            Some('+') => {
                if self.peek_char_eq_consume('=') {
                    self.gen_token(TokenType::PlusAssign, TokenValue::None)
                } else if self.peek_char_eq_consume('+') {
                    self.gen_token(TokenType::PlusPlus, TokenValue::None)
                } else {
                    self.gen_token(TokenType::Plus, TokenValue::None)
                }
            }
            Some('-') => {
                if self.peek_char_eq_consume('=') {
                    self.gen_token(TokenType::MinusAssign, TokenValue::None)
                } else if self.peek_char_eq_consume('-') {
                    self.gen_token(TokenType::MinusMinus, TokenValue::None)
                } else {
                    self.gen_token(TokenType::Minus, TokenValue::None)
                }
            }
            Some('*') => {
                if self.peek_char_eq_consume('=') {
                    self.gen_token(TokenType::AsteriskAssign, TokenValue::None)
                } else {
                    self.gen_token(TokenType::Asterisk, TokenValue::None)
                }
            }
            Some('/') => {
                if self.peek_char_eq_consume('=') {
                    self.gen_token(TokenType::SlashAssign, TokenValue::None)
                } else if self.peek_char_eq_consume('/') {
                    self.gen_token(
                        TokenType::Comment,
                        TokenValue::String(self.read_until_eol().trim_left().into()),
                    )
                } else if self.peek_char_eq_consume('*') {
                    self.gen_token(
                        TokenType::Comment,
                        TokenValue::String(self.read_string_until("*/").trim_left().into()),
                    )
                } else {
                    self.gen_token(TokenType::Slash, TokenValue::None)
                }
            }
            Some('%') => {
                if self.peek_char_eq_consume('=') {
                    self.gen_token(TokenType::PercentAssign, TokenValue::None)
                } else {
                    self.gen_token(TokenType::Percent, TokenValue::None)
                }
            }
            Some('&') => {
                if self.peek_char_eq_consume('&') {
                    self.gen_token(TokenType::And, TokenValue::None)
                } else if self.peek_char_eq_consume('=') {
                    self.gen_token(TokenType::BitAndAssign, TokenValue::None)
                } else {
                    self.gen_token(TokenType::BitAnd, TokenValue::None)
                }
            }
            Some('|') => {
                if self.peek_char_eq_consume('|') {
                    self.gen_token(TokenType::Or, TokenValue::None)
                } else if self.peek_char_eq_consume('=') {
                    self.gen_token(TokenType::BitOrAssign, TokenValue::None)
                } else {
                    self.gen_token(TokenType::BitOr, TokenValue::None)
                }
            }
            Some('^') => {
                if self.peek_char_eq_consume('=') {
                    self.gen_token(TokenType::BitXorAssign, TokenValue::None)
                } else {
                    self.gen_token(TokenType::BitXor, TokenValue::None)
                }
            }
            Some('<') => {
                if self.peek_char_eq_consume('=') {
                    self.gen_token(TokenType::LowerThanEqual, TokenValue::None)
                } else if self.peek_char_eq_consume('<') {
                    if self.peek_char_eq_consume('=') {
                        self.gen_token(TokenType::BitLeftAssign, TokenValue::None)
                    } else {
                        self.gen_token(TokenType::BitLeft, TokenValue::None)
                    }
                } else {
                    self.gen_token(TokenType::LowerThan, TokenValue::None)
                }
            }
            Some('>') => {
                if self.peek_char_eq_consume('=') {
                    self.gen_token(TokenType::GreaterThanEqual, TokenValue::None)
                } else if self.peek_char_eq_consume('>') {
                    if self.peek_char_eq_consume('=') {
                        self.gen_token(TokenType::BitRightAssign, TokenValue::None)
                    } else {
                        self.gen_token(TokenType::BitRight, TokenValue::None)
                    }
                } else {
                    self.gen_token(TokenType::GreaterThan, TokenValue::None)
                }
            }
            Some('!') => {
                if self.peek_char_eq_consume('=') {
                    self.gen_token(TokenType::NotEqual, TokenValue::None)
                } else {
                    self.gen_token(TokenType::Bang, TokenValue::None)
                }
            }
            Some(';') => self.gen_token(TokenType::Semicolon, TokenValue::None),
            Some(':') => self.gen_token(TokenType::Colon, TokenValue::None),
            Some(',') => self.gen_token(TokenType::Comma, TokenValue::None),
            Some('{') => self.gen_token(TokenType::LeftBrace, TokenValue::None),
            Some('}') => self.gen_token(TokenType::RightBrace, TokenValue::None),
            Some('(') => self.gen_token(TokenType::LeftBracket, TokenValue::None),
            Some(')') => self.gen_token(TokenType::RightBracket, TokenValue::None),
            Some('[') => self.gen_token(TokenType::LeftSquare, TokenValue::None),
            Some(']') => self.gen_token(TokenType::RightSquare, TokenValue::None),
            Some('.') => {
                if self.peek_char_eq_consume('.') {
                    if self.peek_char_eq_consume('.') {
                        self.gen_token(TokenType::Elipsis, TokenValue::None)
                    } else {
                        self.gen_token(TokenType::Range, TokenValue::None)
                    }
                } else {
                    self.gen_token(TokenType::Illegal, TokenValue::None)
                }
            }
            Some('#') => self.gen_token(TokenType::Directive, TokenValue::None),

            Some(ch @ _) => {
                if is_letter(ch) {
                    self.read_symbol(ch)
                } else if ch.is_numeric() {
                    self.read_number(ch)
                } else {
                    self.gen_token(TokenType::Illegal, TokenValue::None)
                }
            }

            None => self.gen_token(TokenType::End, TokenValue::None),
        }
    }
}

fn is_letter(ch: char) -> bool {
    ch.is_alphabetic() || ch == '_'
}
