use super::types::*;
use crate::scanner::Scanner;

pub struct Tokenizer<'a> {
    scanner: Scanner<'a>,
    tokens: Vec<Token>,
    row: usize,
    col: usize,
}

impl<'a> Tokenizer<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            scanner: Scanner::new(src),
            tokens: Vec::new(),
            row: 0,
            col: 0,
        }
    }

    pub fn tokenize(mut self) -> Vec<Token> {
        while let Some(c) = self.scanner.peek() {
            match c {
                '(' => self.process_single_symbol(Symbol::LParen,   "("),
                ')' => self.process_single_symbol(Symbol::RParen,   ")"),
                '@' => self.process_single_symbol(Symbol::At,       "@"),
                '$' => self.process_single_symbol(Symbol::Dollar,   "$"),
                '#' => self.process_single_symbol(Symbol::Hash,     "#"),
                '+' => self.process_single_symbol(Symbol::Plus,     "+"),
                '*' => self.process_single_symbol(Symbol::Multiply, "*"),
                '/' => self.process_single_symbol(Symbol::Divide,   "/"),
                '%' => self.process_single_symbol(Symbol::Modulo,   "%"),
                ',' => self.process_single_symbol(Symbol::Comma,    ","),
                '=' => self.process_double_symbol('=', Symbol::DoubleEquals,  Symbol::Equals,      "==", "="),
                ':' => self.process_double_symbol(':', Symbol::DoubleColon,   Symbol::Colon,       "::", ":"),
                '-' => self.process_double_symbol('>', Symbol::Arrow,         Symbol::Minus,       "->", "-"),
                '>' => self.process_double_symbol('=', Symbol::GreaterEquals, Symbol::GreaterThan, ">=", ">"),
                '<' => self.process_double_symbol('=', Symbol::LessEquals,    Symbol::LessThan,    "<=", "<"),
                '|' => self.process_double_symbol('|', Symbol::DoublePipe,    Symbol::Unknown,     "||", "|"),
                '0'..='9' => self.process_numerical_lit(),
                'a'..='z' | 'A'..='Z' | '_' => self.process_str_lit_or_ident(),
                c => self.process_single_symbol(Symbol::Unknown, c.encode_utf8(&mut [0; 4])),
            }
        }

        self.tokens
    }

    fn process_str_lit_or_ident(&mut self) {
        let mut lit = self.advance().unwrap().to_string();

        while let Some(c) = self.scanner.peek() {
            match c {
                'a'..='z' | 'A'..='Z' | '_' => {
                    lit.push(self.advance().unwrap());
                },
                _ => break,
            }
        }

        match lit.as_str() {
            "Int"    => self.push_token(TokenVariant::Symbol(Symbol::IntType),    &lit),
            "Float"  => self.push_token(TokenVariant::Symbol(Symbol::FloatType),  &lit),
            "Bool"   => self.push_token(TokenVariant::Symbol(Symbol::BoolType),   &lit),
            "String" => self.push_token(TokenVariant::Symbol(Symbol::StringType), &lit),
            "Func"   => self.push_token(TokenVariant::Symbol(Symbol::FuncType),   &lit),
            "true" | "false" => self.push_token(TokenVariant::Literal(Literal::Bool), &lit),
            _ => self.push_token(TokenVariant::Identifier, &lit),
        }
    }

    fn process_numerical_lit(&mut self) {
        let mut lit = self.advance().unwrap().to_string();
        let mut is_float = false;

        while let Some(c) = self.scanner.peek() {
            match c {
                '0'..='9' => {
                    lit.push(self.advance().unwrap());
                },
                '.' => {
                    if is_float {
                        break;
                    }

                    lit.push(self.advance().unwrap());
                    is_float = true;
                },
                _ => break,
            }
        }
                    
        if is_float {
            self.push_token(TokenVariant::Literal(Literal::Float), &lit);
        } else {
            self.push_token(TokenVariant::Literal(Literal::Int), &lit);
        }
    }

    fn process_double_symbol(&mut self, c: char, sym_a: Symbol, sym_b: Symbol, lit_a: &str, lit_b: &str) {
        self.advance();
        if self.scanner.peek() == Some(c) {
            self.push_token(TokenVariant::Symbol(sym_a), lit_a);
            self.advance();
        } else {
            self.push_token(TokenVariant::Symbol(sym_b), lit_b);
        }
    }

    fn process_single_symbol(&mut self, sym: Symbol, lit: &str) {
        self.push_token(TokenVariant::Symbol(sym), lit);
        self.advance();
    }

    fn push_token(&mut self, var: TokenVariant, rep: &str) {
        self.tokens.push(Token {
            var,
            lit: String::from(rep),
            row: self.row,
            col: self.col,
        });
    }

    fn advance(&mut self) -> Option<char> {
        let Some(c) = self.scanner.eat() else { return None };
        if c == '\n' {
            self.row += 1;
            self.col = 0;
        } else {
            self.col += 1;
        }
        Some(c)
    }
}
