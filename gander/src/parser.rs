use std::iter::Peekable;

use crate::{
    ast::*,
    token::Token,
    tokenizer::Tokenizer
};

#[derive(Debug)]
struct Parser<'a> {
    tokenizer: Peekable<Tokenizer<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            tokenizer: Tokenizer::new(src).peekable(),
        }
    }

    pub fn parse_program(&mut self) -> Program {
        let mut items = Vec::new();

        while let Some(_) = self.peek() {
            items.push(self.parse_item());
        }
        
        Program { items }
    }

    fn parse_item(&mut self) -> Item {
        Item::Expression(Expression::Primary(PrimaryExpr::Literal(Literal::Int(0))))
    }

    fn peek(&mut self) -> Option<&Token> {
        self.tokenizer.peek()
    }

    fn eat(&mut self) -> Option<Token> {
        self.tokenizer.next()
    }
}
