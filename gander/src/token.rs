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
        match &self.var {
            TokenVariant::Symbol(_) => write!(f, "{} @ {}:{}", self.var, self.row, self.col),
            _ => write!(f, "{}({}) @ {}:{}", self.var, self.lit, self.row, self.col),
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
    IntType,
    FloatType,
    BoolType,
    StringType,
    FuncType,
    Unknown,
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Symbol::*;

        let s = match self {
            LParen => "(",
            RParen => ")",
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
            IntType => "Int",
            FloatType => "Float",
            BoolType => "Bool",
            StringType => "String",
            FuncType => "Func",
            Unknown => "Unknown",
        };

        write!(f, "{}", s)
    }
}
