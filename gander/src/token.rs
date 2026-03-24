use core::fmt;

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub var: TokenVariant,
    pub lit: Option<String>,
    pub row: usize,
    pub col: usize,
}

impl Token {
    pub fn new(var: TokenVariant, lit: Option<&str>, row: usize, col: usize) -> Self {
        Self {
            var,
            lit: lit.map(|s| s.to_string()),
            row,
            col,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.lit {
            Some(l) => write!(f, "{}({}) @ {}:{}", self.var, l, self.row, self.col),
            None => write!(f, "{} @ {}:{}", self.var, self.row, self.col),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenVariant {
    Literal(Literal),
    Symbol(Symbol),
    Identifier,
}

impl fmt::Display for TokenVariant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use TokenVariant::*;

        match self {
            Literal(i) => write!(f, "Literal<{}>", i),
            Symbol(i) => write!(f, "Symbol({})", i),
            Identifier => write!(f, "Identifier"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    Int,
    Float,
    Bool,
    String,
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Literal::*;

        let l = match self {
            Int => "Int",
            Float => "Float",
            Bool => "Bool",
            String => "String",
        };

        write!(f, "{}", l)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Symbol {
    LParen,
    RParen,
    LCurl,
    RCurl,
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
    Period,
    Exclaim,
    IntType,
    BoolType,
    UnitType,
    FloatType,
    StringType,
    Unknown,
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Symbol::*;

        let s = match self {
            LParen => "(",
            RParen => ")",
            LCurl => "{",
            RCurl => "}",
            Equals => "=",
            DoubleEquals => "==",
            NotEquals => "!=",
            GreaterEquals => ">=",
            LessEquals => "<=",
            GreaterThan => ">",
            LessThan => "<",
            Arrow => "->",
            At => "@",
            Dollar => "$",
            Hash => "#",
            DoubleColon => "::",
            Colon => ":",
            Plus => "+",
            Minus => "-",
            Multiply => "*",
            Divide => "/",
            Modulo => "%",
            Comma => ",",
            Period => ".",
            Exclaim => "!",
            IntType => "Int",
            BoolType => "Bool",
            UnitType => "Unit",
            FloatType => "Float",
            StringType => "String",
            Unknown => "Unknown",
        };

        write!(f, "{}", s)
    }
}
