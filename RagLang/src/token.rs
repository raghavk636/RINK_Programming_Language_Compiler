#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Let,
    Print,
    Identifier(String),
    Number(i64),
    Equals,
    Plus,
    Minus,
    Star,
    Slash,
    Eq,        // ==
    NotEq,     // !=
    Less,      // <
    LessEq,    // <=
    Greater,   // >
    GreaterEq, // >=
    LParen,
    RParen,
    Semicolon,
    EOF,
    Illegal(char),
    If,
    Else,
    LBrace,
    RBrace,
    While,
}
