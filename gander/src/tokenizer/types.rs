use thiserror::Error;

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub var: TokenVariant,
    pub lit: String,
    pub row: usize,
    pub col: usize
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenVariant {
    Literal(Literal),
    Symbol(Symbol),
    Identifier
}

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    Int,
    Float,
    Bool,
    String,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Symbol {
    LParen,
    RParen,
    Equals,
    DoubleEquals,
    NotEquals,
    GreaterEquals,
    LessEquals,
    GreaterThan,
    LessThan,
    Arrow,
    At,
    Dollar,
    Hash,
    DoubleColon,
    Colon,
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    Comma
}

#[derive(Error, Debug)]
pub enum TokenizerError {
    #[error("unknown tokenizer error at line {0}")]
    Unknown(usize)
}