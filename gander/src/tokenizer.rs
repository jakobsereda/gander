use crate::token::*;
use crate::scanner::Scanner;

#[derive(Clone, Debug)]
pub struct Tokenizer<'a> {
    scanner: Scanner<'a>,
    row: usize,
    col: usize,
}

impl<'a> Tokenizer<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            scanner: Scanner::new(src),
            row: 1,
            col: 1,
        }
    }

    fn advance(&mut self) -> Option<char> {
        let c = self.scanner.eat()?;

        if c == '\n' {
            self.row += 1;
            self.col = 1;
        } else {
            self.col += 1;
        }

        Some(c)
    }

    fn process_single_symbol(&self, sym: Symbol) -> Token {
        Token::new(TokenVariant::Symbol(sym), None, self.row, self.col)
    }

    fn process_double_symbol(&mut self, cmp: char, sym_a: Symbol, sym_b: Symbol) -> Token {
        if self.scanner.peek() == Some(cmp) {
            let ret = self.process_single_symbol(sym_a);
            self.advance();
            ret
        } else {
            self.process_single_symbol(sym_b)
        }
    }

    fn process_comments(&mut self) -> Option<Token> {
        if self.scanner.peek() == Some('|') {
            while let Some(c) = self.advance() {
                if c == '\n' {
                    break;
                }
            }
            None
        } else {
            Some(Token::new(
                TokenVariant::Symbol(Symbol::Unknown), 
                Some("|"), 
                self.row, 
                self.col
            ))
        }
    }

    fn process_numerical_lit(&mut self, origin: char) -> Token {
        let mut lit = origin.to_string();
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
            Token::new(
                TokenVariant::Literal(Literal::Float),
                Some(&lit),
                self.row,
                self.col
            )
        } else {
            Token::new(
                TokenVariant::Literal(Literal::Int),
                Some(&lit),
                self.row,
                self.col
            )
        }
    }

    fn process_str_lit_or_ident(&mut self, origin: char) -> Token {
        let mut lit = origin.to_string();

        while let Some(c) = self.scanner.peek() {
            match c {
                'a'..='z' | 'A'..='Z' | '_' | '0'..='9' => {
                   lit.push(self.advance().unwrap());
                },
                _ => break,
            }
        }

        let var = match lit.as_str() {
            "Int"    => TokenVariant::Symbol(Symbol::IntType),
            "Bool"   => TokenVariant::Symbol(Symbol::BoolType),
            "Func"   => TokenVariant::Symbol(Symbol::FuncType),
            "Float"  => TokenVariant::Symbol(Symbol::FloatType),
            "String" => TokenVariant::Symbol(Symbol::StringType),
            "true" | "false" => TokenVariant::Literal(Literal::Bool),
            _ => TokenVariant::Identifier,
        };

        match var {
            TokenVariant::Symbol(_) => Token::new(var, None, self.row, self.col),
            _ => Token::new(var, Some(&lit), self.row, self.col),
        }
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        loop {
            let c = self.advance()?;

            let token = match c {
                ' ' | '\t' | '\n' => None,
                '|' => self.process_comments(),
                '0'..='9' => Some(self.process_numerical_lit(c)),
                'a'..='z' | 'A'..='Z' | '_' => Some(self.process_str_lit_or_ident(c)),
                '@' => Some(self.process_single_symbol(Symbol::At)),
                '#' => Some(self.process_single_symbol(Symbol::Hash)),
                '+' => Some(self.process_single_symbol(Symbol::Plus)),
                ',' => Some(self.process_single_symbol(Symbol::Comma)),
                '(' => Some(self.process_single_symbol(Symbol::LParen)),
                ')' => Some(self.process_single_symbol(Symbol::RParen)),
                '$' => Some(self.process_single_symbol(Symbol::Dollar)),
                '/' => Some(self.process_single_symbol(Symbol::Divide)),
                '%' => Some(self.process_single_symbol(Symbol::Modulo)),
                '.' => Some(self.process_single_symbol(Symbol::Period)),
                '*' => Some(self.process_single_symbol(Symbol::Multiply)),
                '-' => Some(self.process_double_symbol('>', Symbol::Arrow,         Symbol::Minus)),
                ':' => Some(self.process_double_symbol(':', Symbol::DoubleColon,   Symbol::Colon)),
                '=' => Some(self.process_double_symbol('=', Symbol::DoubleEquals,  Symbol::Equals)),
                '!' => Some(self.process_double_symbol('=', Symbol::NotEquals,     Symbol::Exclaim)),
                '<' => Some(self.process_double_symbol('=', Symbol::LessEquals,    Symbol::LessThan)),
                '>' => Some(self.process_double_symbol('=', Symbol::GreaterEquals, Symbol::GreaterThan)),
                c => {
                    let mut buf = [0; 4];
                    Some(Token::new(
                        TokenVariant::Symbol(Symbol::Unknown),
                        Some(c.encode_utf8(&mut buf)),
                        self.row,
                        self.col
                    ))
                },
            };

            if token.is_some() {
                return token;
            }
        }
    }
}
