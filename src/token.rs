#[derive(Debug, PartialEq)]
pub enum Token {
    Illegal,
    End,

    Comment(String),

    // -
    // Symbols
    // -
    Equal,            // ==
    Assign,           // =
    Plus,             // +
    PlusPlus,         // ++
    PlusAssign,       // +=
    Minus,            // -
    MinusMinus,       // --
    MinusAssign,      // -=
    Asterisk,         // *
    AsteriskAssign,   // *=
    Slash,            // /
    SlashAssign,      // /=
    Percent,          // %
    PercentAssign,    // %=
    And,              // &&
    BitAnd,           // &
    BitAndAssign,     // &=
    Or,               // ||
    BitOr,            // |
    BitOrAssign,      // |=
    BitXor,           // ^
    BitXorAssign,     // ^=
    LowerThan,        // <
    LowerThanEqual,   // <=
    BitLeft,          // <<
    BitLeftAssign,    // <<=
    GreaterThan,      // >
    GreaterThanEqual, // >=
    BitRight,         // >>
    BitRightAssign,   // >>=
    NotEqual,         // !=
    Bang,             // !
    Semicolon,        // ;
    Comma,            // ,
    LeftBrace,        // {
    RightBrace,       // }
    LeftBracket,      // (
    RightBracket,     // )
    LeftSquare,       // [
    RightSquare,      // ]
    Elipsis,          // ...
    Range,            // ..
    Directive,        // #

    // -
    // Keywords - declaration/definition
    // -
    Const,    // const
    New,      // new
    Static,   // static
    Stock,    // stock
    Forward,  // forward
    Public,   // public
    Native,   // native
    Operator, // operator
    Char,     // char
    Enum,     // enum
    State,    // state

    // -
    // Keywords - control flow
    // -
    If,       // if
    Else,     // else
    Switch,   // switch
    Case,     // case
    Default,  // default
    For,      // for
    While,    // while
    Do,       // do
    Break,    // break
    Continue, // continue
    Goto,     // goto
    Return,   // return
    Sizeof,   // sizeof
    Tagof,    // tagof
    Emit,     // __emit

    // -
    // Patterns
    // -
    Integer(i32),    // integer number
    Float(f32),      // floating point number
    Symbol(String),  // a-zA-Z0-9_@
    Label(String),   // a-zA-Z0-9_
    Literal(String), // ".*"
}

impl Default for Token {
    fn default() -> Token {
        Token::Illegal
    }
}

pub fn lookup_keyword(kw: &str) -> Token {
    match kw {
        "const" => (Token::Const),
        "new" => (Token::New),
        "static" => (Token::Static),
        "stock" => (Token::Stock),
        "forward" => (Token::Forward),
        "public" => (Token::Public),
        "native" => (Token::Native),
        "operator" => (Token::Operator),
        "char" => (Token::Char),
        "enum" => (Token::Enum),
        "state" => (Token::State),

        "if" => (Token::If),
        "else" => (Token::Else),
        "switch" => (Token::Switch),
        "case" => (Token::Case),
        "default" => (Token::Default),
        "for" => (Token::For),
        "while" => (Token::While),
        "do" => (Token::Do),
        "break" => (Token::Break),
        "continue" => (Token::Continue),
        "goto" => (Token::Goto),
        "return" => (Token::Return),
        "sizeof" => (Token::Sizeof),
        "tagof" => (Token::Tagof),
        "__emit" => (Token::Emit),

        _ => Token::Symbol(String::from(kw)),
    }
}
