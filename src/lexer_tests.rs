#[cfg(test)]
use crate::lexer::Lexer;
#[cfg(test)]
use crate::token::Token;

#[test]
fn lex_comment_line() {
    assert_eq!(
        Lexer::new("// comment").lex(),
        vec![Token::Comment(String::from("comment")),],
    );
}

#[test]
fn lex_comment_block() {
    assert_eq!(
        Lexer::new("/* comment */").lex(),
        vec![Token::Comment(String::from("comment")),],
    );
}

#[test]
fn lex_comment_block_multi() {
    assert_eq!(
        Lexer::new(
            "/*
comment on
multiple lines
*/"
        )
        .lex(),
        vec![Token::Comment(String::from("comment on\nmultiple lines")),],
    );
}

#[test]
fn lex_cell_declaration() {
    assert_eq!(
        Lexer::new("new x = 5;").lex(),
        vec![
            Token::New,
            Token::Symbol(String::from("x")),
            Token::Assign,
            Token::Integer(5),
            Token::Semicolon,
        ],
    );
}

#[test]
fn lex_float_decl() {
    assert_eq!(
        Lexer::new("new Float:x = 5.5;").lex(),
        vec![
            Token::New,
            Token::Symbol(String::from("Float")),
            Token::Colon,
            Token::Symbol(String::from("x")),
            Token::Assign,
            Token::Float(5.5),
            Token::Semicolon,
        ],
        "float declaration"
    );
}

#[test]
fn lex_array_decl_autosize() {
    assert_eq!(
        Lexer::new("new x[] = {1, 2, 3};").lex(),
        vec![
            Token::New,
            Token::Symbol(String::from("x")),
            Token::LeftSquare,
            Token::RightSquare,
            Token::Assign,
            Token::LeftBrace,
            Token::Integer(1),
            Token::Comma,
            Token::Integer(2),
            Token::Comma,
            Token::Integer(3),
            Token::RightBrace,
            Token::Semicolon,
        ]
    );
}

#[test]
fn lex_array_decl() {
    assert_eq!(
        Lexer::new("new x[4] = {1, 2, 3};").lex(),
        vec![
            Token::New,
            Token::Symbol(String::from("x")),
            Token::LeftSquare,
            Token::Integer(4),
            Token::RightSquare,
            Token::Assign,
            Token::LeftBrace,
            Token::Integer(1),
            Token::Comma,
            Token::Integer(2),
            Token::Comma,
            Token::Integer(3),
            Token::RightBrace,
            Token::Semicolon,
        ],
        "array declaration specific size"
    );
}

#[test]
fn lex_basic_script() {
    assert_eq!(
        Lexer::new(
            "
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
"
        )
        .lex(),
        vec![
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
        ]
    );
}
