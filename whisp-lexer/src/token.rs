#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Keywords
    Array,
    Let,
    If,
    Elif,
    Else,
    While,
    For,
    In,
    Def,
    Return,

    // Literals and Identifiers
    Identifier(String),
    Int(i32),
    Bool(bool),
    String(String),

    // Arithmetic Operators
    Add,          // +
    Sub,          // -
    Mul,          // *
    Div,          // /
    Mod,          // %

    // Assignment & Comparison
    Assign,       // =
    Equal,        // ==
    GreaterThan,  // >
    LessThan,     // <
    GreaterEqual, // >=
    LessEqual,    // <=

    // Logical Operators
    And,          // and
    Or,           // or

    // Punctuation
    LParen,       // (
    RParen,       // )
    LBrace,       // {
    RBrace,       // }
    LBracket,     // [
    RBracket,     // ]
    Comma,        // ,
    Semicolon,    // ;

    // End of file/input
    Eof,
}
