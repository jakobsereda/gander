use core::fmt;

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub var: TokenVariant,
    pub lit: String,
    pub row: usize,
    pub col: usize,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} @ {}:{}", self.lit, self.row, self.col)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenVariant {
    Literal(Literal),
    Symbol(Symbol),
    Identifier,
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
    Comma,
    DoublePipe,
    IntType,
    FloatType,
    BoolType,
    StringType,
    FuncType,
    Unknown,
}
