use std::iter::Peekable;
use std::str::Chars;

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

    fn read_char(&mut self) -> Option<char> {
        self.input.next()
    }

    fn peek_char(&mut self) -> Option<&char> {
        self.input.peek()
    }

    fn peek_char_eq(&mut self, ch: char) -> bool {
        match self.peek_char() {
            Some(&peek_ch) => peek_ch == ch,
            None => false,
        }
    }

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

    fn read_identifier(&mut self, first: char) -> String {
        let mut ident = String::new();
        ident.push(first);

        while self.peek_is_letter() {
            ident.push(self.read_char().unwrap());
        }

        ident
    }

    fn read_number(&mut self, first: char) -> i32 {
        let mut number = String::new();
        number.push(first);

        while let Some(&c) = self.peek_char() {
            if !c.is_numeric() {
                break;
            }
            number.push(self.read_char().unwrap());
        }

        number.parse().unwrap()
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
                    let literal = self.read_identifier(ch);
                    token::lookup_keyword(&literal)
                } else if ch.is_numeric() {
                    Token::Integer(self.read_number(ch))
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

#[test]
fn parse_test_1() {
    let input = "
// Comment
#include <a_samp>

main() {
    new a;
    if(a == 3) {
        a++;
    } else if(a != 3) {
        a--;
    } else {
        a = 0;
    }
}
";

    let want = vec![
        Token::Comment(String::from("Comment")),
        Token::Directive,
        Token::Symbol(String::from("include")),
        Token::LowerThan,
        Token::Symbol(String::from("a_samp")),
        Token::GreaterThan,
        Token::Symbol(String::from("main")),
        Token::LeftBracket,
        Token::RightBracket,
        Token::LeftBrace,
        Token::New,
        Token::Symbol(String::from("a")),
        Token::Semicolon,
        Token::If,
        Token::LeftBracket,
        Token::Symbol(String::from("a")),
        Token::Equal,
        Token::Integer(3),
        Token::RightBracket,
        Token::LeftBrace,
        Token::Symbol(String::from("a")),
        Token::PlusPlus,
        Token::Semicolon,
        Token::RightBrace,
        Token::Else,
        Token::If,
        Token::LeftBracket,
        Token::Symbol(String::from("a")),
        Token::NotEqual,
        Token::Integer(3),
        Token::RightBracket,
        Token::LeftBrace,
        Token::Symbol(String::from("a")),
        Token::MinusMinus,
        Token::Semicolon,
        Token::RightBrace,
        Token::Else,
        Token::LeftBrace,
        Token::Symbol(String::from("a")),
        Token::Assign,
        Token::Integer(0),
        Token::Semicolon,
        Token::RightBrace,
        Token::RightBrace,
    ];
    let mut got = Vec::new();

    let mut l = Lexer::new(input);
    loop {
        let tok = l.next_token();
        if tok == Token::End {
            break;
        }
        got.push(tok)
    }

    assert_eq!(got, want);
}
