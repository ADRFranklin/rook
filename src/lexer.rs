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
                if self.peek_char_eq('=') {
                    Token::Equal
                } else {
                    Token::Assign
                }
            }
            Some('+') => {
                if self.peek_char_eq('=') {
                    Token::PlusAssign
                } else if self.peek_char_eq('+') {
                    Token::PlusPlus
                } else {
                    Token::Plus
                }
            }
            Some('-') => {
                if self.peek_char_eq('=') {
                    Token::MinusAssign
                } else if self.peek_char_eq('-') {
                    Token::MinusMinus
                } else {
                    Token::Minus
                }
            }
            Some('*') => {
                if self.peek_char_eq('=') {
                    Token::AsteriskAssign
                } else {
                    Token::Asterisk
                }
            }
            Some('/') => {
                if self.peek_char_eq('=') {
                    Token::SlashAssign
                } else {
                    Token::Slash
                }
            }
            Some('%') => {
                if self.peek_char_eq('=') {
                    Token::PercentAssign
                } else {
                    Token::Percent
                }
            }
            Some('&') => {
                if self.peek_char_eq('&') {
                    Token::And
                } else if self.peek_char_eq('=') {
                    Token::BitAndAssign
                } else {
                    Token::BitAnd
                }
            }
            Some('|') => {
                if self.peek_char_eq('|') {
                    Token::Or
                } else if self.peek_char_eq('=') {
                    Token::BitOrAssign
                } else {
                    Token::BitOr
                }
            }
            Some('^') => {
                if self.peek_char_eq('=') {
                    Token::BitXorAssign
                } else {
                    Token::BitXor
                }
            }
            Some('<') => {
                if self.peek_char_eq('=') {
                    Token::LowerThanEqual
                } else if self.peek_char_eq('<') {
                    if self.peek_char_eq('=') {
                        Token::BitLeftAssign
                    } else {
                        Token::BitLeft
                    }
                } else {
                    Token::LowerThan
                }
            }
            Some('>') => {
                if self.peek_char_eq('=') {
                    Token::GreaterThanEqual
                } else if self.peek_char_eq('>') {
                    if self.peek_char_eq('=') {
                        Token::BitRightAssign
                    } else {
                        Token::BitRight
                    }
                } else {
                    Token::GreaterThan
                }
            }
            Some('!') => {
                if self.peek_char_eq('=') {
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
                if self.peek_char_eq('.') {
                    if self.peek_char_eq('.') {
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

    let tests = vec![
        Token::Directive,
        Token::Symbol(String::from("include")),
        Token::Symbol(String::from("main")),
        Token::LeftBracket,
        Token::RightBracket,
    ];

    let mut l = Lexer::new(input);
    for t in tests {
        let tok = l.next_token();
        assert_eq!(tok, t);
    }
}
