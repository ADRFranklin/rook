use std::iter::Peekable;
use std::str::Chars;

use crate::ring::Ring;
use crate::token;
use crate::token::Token;

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &str) -> Lexer {
        Lexer {
            input: input.chars().peekable(),
        }
    }

    pub fn lex(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        loop {
            let tok = self.next_token();
            if tok == Token::End {
                break;
            }
            tokens.push(tok)
        }
        tokens
    }

    fn read_char(&mut self) -> Option<char> {
        self.input.next()
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
        token::lookup_keyword(&ident)
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
            Token::Float(number.parse().unwrap())
        } else {
            Token::Integer(number.parse().unwrap())
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        match self.read_char() {
            Some('=') => {
                if self.peek_char_eq_consume('=') {
                    Token::Equal
                } else {
                    Token::Assign
                }
            }
            Some('+') => {
                if self.peek_char_eq_consume('=') {
                    Token::PlusAssign
                } else if self.peek_char_eq_consume('+') {
                    Token::PlusPlus
                } else {
                    Token::Plus
                }
            }
            Some('-') => {
                if self.peek_char_eq_consume('=') {
                    Token::MinusAssign
                } else if self.peek_char_eq_consume('-') {
                    Token::MinusMinus
                } else {
                    Token::Minus
                }
            }
            Some('*') => {
                if self.peek_char_eq_consume('=') {
                    Token::AsteriskAssign
                } else {
                    Token::Asterisk
                }
            }
            Some('/') => {
                if self.peek_char_eq_consume('=') {
                    Token::SlashAssign
                } else if self.peek_char_eq_consume('/') {
                    Token::Comment(self.read_until_eol().trim_left().into())
                } else if self.peek_char_eq_consume('*') {
                    Token::Comment(self.read_string_until("*/").trim_left().into())
                } else {
                    Token::Slash
                }
            }
            Some('%') => {
                if self.peek_char_eq_consume('=') {
                    Token::PercentAssign
                } else {
                    Token::Percent
                }
            }
            Some('&') => {
                if self.peek_char_eq_consume('&') {
                    Token::And
                } else if self.peek_char_eq_consume('=') {
                    Token::BitAndAssign
                } else {
                    Token::BitAnd
                }
            }
            Some('|') => {
                if self.peek_char_eq_consume('|') {
                    Token::Or
                } else if self.peek_char_eq_consume('=') {
                    Token::BitOrAssign
                } else {
                    Token::BitOr
                }
            }
            Some('^') => {
                if self.peek_char_eq_consume('=') {
                    Token::BitXorAssign
                } else {
                    Token::BitXor
                }
            }
            Some('<') => {
                if self.peek_char_eq_consume('=') {
                    Token::LowerThanEqual
                } else if self.peek_char_eq_consume('<') {
                    if self.peek_char_eq_consume('=') {
                        Token::BitLeftAssign
                    } else {
                        Token::BitLeft
                    }
                } else {
                    Token::LowerThan
                }
            }
            Some('>') => {
                if self.peek_char_eq_consume('=') {
                    Token::GreaterThanEqual
                } else if self.peek_char_eq_consume('>') {
                    if self.peek_char_eq_consume('=') {
                        Token::BitRightAssign
                    } else {
                        Token::BitRight
                    }
                } else {
                    Token::GreaterThan
                }
            }
            Some('!') => {
                if self.peek_char_eq_consume('=') {
                    Token::NotEqual
                } else {
                    Token::Bang
                }
            }
            Some(';') => Token::Semicolon,
            Some(':') => Token::Colon,
            Some(',') => Token::Comma,
            Some('{') => Token::LeftBrace,
            Some('}') => Token::RightBrace,
            Some('(') => Token::LeftBracket,
            Some(')') => Token::RightBracket,
            Some('[') => Token::LeftSquare,
            Some(']') => Token::RightSquare,
            Some('.') => {
                if self.peek_char_eq_consume('.') {
                    if self.peek_char_eq_consume('.') {
                        Token::Elipsis
                    } else {
                        Token::Range
                    }
                } else {
                    Token::Illegal
                }
            }
            Some('#') => Token::Directive,

            Some(ch @ _) => {
                if is_letter(ch) {
                    self.read_symbol(ch)
                } else if ch.is_numeric() {
                    self.read_number(ch)
                } else {
                    token::Token::Illegal
                }
            }

            None => Token::End,
        }
    }
}

fn is_letter(ch: char) -> bool {
    ch.is_alphabetic() || ch == '_'
}
