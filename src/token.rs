#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub value: TokenValue,
    pub line: i32,
    pub column: i32,
    pub range: i32,
}

#[derive(Debug)]
pub enum TokenValue {
    None,
    String(String),
    Integer(i32),
    Float(f32),
}

impl TokenValue {
    pub fn len(&self) -> usize {
        match self {
            TokenValue::None => 0,
            TokenValue::String(v) => v.len(),
            TokenValue::Integer(v) => v.to_string().len(),
            TokenValue::Float(v) => v.to_string().len(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
    Illegal,
    End,

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
    Colon,            // :
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
    Integer, // integer number
    Float,   // floating point number
    Symbol,  // a-zA-Z0-9_@
    Label,   // a-zA-Z0-9_
    Literal, // ".*"
    Comment,
}

impl Default for TokenType {
    fn default() -> TokenType {
        TokenType::Illegal
    }
}

impl ToString for TokenType {
    fn to_string(&self) -> String {
        match self {
            TokenType::Illegal => String::from("Illegal"),
            TokenType::End => String::from("End"),
            TokenType::Equal => String::from("=="),
            TokenType::Assign => String::from("="),
            TokenType::Plus => String::from("+"),
            TokenType::PlusPlus => String::from("++"),
            TokenType::PlusAssign => String::from("+="),
            TokenType::Minus => String::from("-"),
            TokenType::MinusMinus => String::from("--"),
            TokenType::MinusAssign => String::from("-="),
            TokenType::Asterisk => String::from("*"),
            TokenType::AsteriskAssign => String::from("*="),
            TokenType::Slash => String::from("/"),
            TokenType::SlashAssign => String::from("/="),
            TokenType::Percent => String::from("%"),
            TokenType::PercentAssign => String::from("%="),
            TokenType::And => String::from("&&"),
            TokenType::BitAnd => String::from("&"),
            TokenType::BitAndAssign => String::from("&="),
            TokenType::Or => String::from("||"),
            TokenType::BitOr => String::from("|"),
            TokenType::BitOrAssign => String::from("|="),
            TokenType::BitXor => String::from("^"),
            TokenType::BitXorAssign => String::from("^="),
            TokenType::LowerThan => String::from("<"),
            TokenType::LowerThanEqual => String::from("<="),
            TokenType::BitLeft => String::from("<<"),
            TokenType::BitLeftAssign => String::from("<<="),
            TokenType::GreaterThan => String::from(">"),
            TokenType::GreaterThanEqual => String::from(">="),
            TokenType::BitRight => String::from(">>"),
            TokenType::BitRightAssign => String::from(">>="),
            TokenType::NotEqual => String::from("!="),
            TokenType::Bang => String::from("!"),
            TokenType::Colon => String::from(":"),
            TokenType::Semicolon => String::from(";"),
            TokenType::Comma => String::from(","),
            TokenType::LeftBrace => String::from("{"),
            TokenType::RightBrace => String::from("}"),
            TokenType::LeftBracket => String::from("("),
            TokenType::RightBracket => String::from(")"),
            TokenType::LeftSquare => String::from("["),
            TokenType::RightSquare => String::from("]"),
            TokenType::Elipsis => String::from("..."),
            TokenType::Range => String::from(".."),
            TokenType::Directive => String::from("#"),
            TokenType::Const => String::from("const"),
            TokenType::New => String::from("new"),
            TokenType::Static => String::from("static"),
            TokenType::Stock => String::from("stock"),
            TokenType::Forward => String::from("forward"),
            TokenType::Public => String::from("public"),
            TokenType::Native => String::from("native"),
            TokenType::Operator => String::from("operator"),
            TokenType::Char => String::from("char"),
            TokenType::Enum => String::from("enum"),
            TokenType::State => String::from("state"),
            TokenType::If => String::from("if"),
            TokenType::Else => String::from("else"),
            TokenType::Switch => String::from("switch"),
            TokenType::Case => String::from("case"),
            TokenType::Default => String::from("default"),
            TokenType::For => String::from("for"),
            TokenType::While => String::from("while"),
            TokenType::Do => String::from("do"),
            TokenType::Break => String::from("break"),
            TokenType::Continue => String::from("continue"),
            TokenType::Goto => String::from("goto"),
            TokenType::Return => String::from("return"),
            TokenType::Sizeof => String::from("sizeof"),
            TokenType::Tagof => String::from("tagof"),
            TokenType::Emit => String::from("__emit"),
            TokenType::Integer => String::from("Integer"),
            TokenType::Float => String::from("Float"),
            TokenType::Symbol => String::from("Symbol"),
            TokenType::Label => String::from("Label"),
            TokenType::Literal => String::from("Literal"),
            TokenType::Comment => String::from("Comment"),
        }
    }
}

pub fn lookup_keyword(kw: &str) -> TokenType {
    match kw {
        "const" => TokenType::Const,
        "new" => TokenType::New,
        "static" => TokenType::Static,
        "stock" => TokenType::Stock,
        "forward" => TokenType::Forward,
        "public" => TokenType::Public,
        "native" => TokenType::Native,
        "operator" => TokenType::Operator,
        "char" => TokenType::Char,
        "enum" => TokenType::Enum,
        "state" => TokenType::State,

        "if" => TokenType::If,
        "else" => TokenType::Else,
        "switch" => TokenType::Switch,
        "case" => TokenType::Case,
        "default" => TokenType::Default,
        "for" => TokenType::For,
        "while" => TokenType::While,
        "do" => TokenType::Do,
        "break" => TokenType::Break,
        "continue" => TokenType::Continue,
        "goto" => TokenType::Goto,
        "return" => TokenType::Return,
        "sizeof" => TokenType::Sizeof,
        "tagof" => TokenType::Tagof,
        "__emit" => TokenType::Emit,

        _ => TokenType::Symbol,
    }
}
