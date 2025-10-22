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

    pub fn tokenize(mut self) -> Result<Vec<Token>, TokenizerError> {
        while let Some(c) = self.scanner.peek() {
            if self.process_single_symbol(c) {
                continue;
            }

            match c {
                '=' => self.process_double_symbol('=', Symbol::DoubleEquals, Symbol::Equals, "==", "="),
                ':' => self.process_double_symbol(':', Symbol::DoubleColon, Symbol::Colon, "::", ":"),
                '-' => self.process_double_symbol('>', Symbol::Arrow, Symbol::Minus, "->", "-"),
                '>' => self.process_double_symbol('=', Symbol::GreaterEquals, Symbol::GreaterThan, ">=", ">"),
                '<' => self.process_double_symbol('=', Symbol::LessEquals, Symbol::LessThan, "<=", "<"),
                _ => return Err(TokenizerError::Unknown(self.row))
            }
        }

        Ok(self.tokens)
    }

    fn process_double_symbol(&mut self, c: char, sym_a: Symbol, sym_b: Symbol, lit_a: &str, lit_b: &str) {
        self.advance();
        if self.scanner.peek() == Some(c) {
            self.push_token(sym_a, lit_a);
            self.advance();
        } else {
            self.push_token(sym_b, lit_b);
        }
    }

    fn process_single_symbol(&mut self, c: char) -> bool {
        let s = match c {
            '(' => Some(Symbol::LParen),
            ')' => Some(Symbol::RParen),
            '@' => Some(Symbol::At),
            '$' => Some(Symbol::Dollar),
            '#' => Some(Symbol::Hash),
            '+' => Some(Symbol::Plus),
            '*' => Some(Symbol::Multiply),
            '/' => Some(Symbol::Divide),
            '%' => Some(Symbol::Modulo),
            ',' => Some(Symbol::Comma),
            _ => None,
        };

        if let Some(symbol) = s {
            self.push_token(symbol, &c.to_string());
            self.advance();
            true
        } else {
            false
        }
    }

    fn push_token(&mut self, symbol: Symbol, rep: &str) {
        self.tokens.push(Token {
            var: TokenVariant::Symbol(symbol),
            lit: String::from(rep),
            row: self.row,
            col: self.col,
        });
    }

    fn advance(&mut self) {
        let Some(c) = self.scanner.eat() else { return };
        if c == '\n' {
            self.row += 1;
            self.col = 0;
        } else {
            self.col += 1;
        }
    }
}
