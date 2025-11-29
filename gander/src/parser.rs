use crate::{
    ast::*,
    token::{TokenVariant, Symbol},
    tokenbuffer::TokenBuffer,
    tokenizer::Tokenizer
};

#[derive(Debug)]
struct Parser<'a> {
    tokens: TokenBuffer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            tokens: TokenBuffer::new(Tokenizer::new(src)),
        }
    }

    pub fn parse_program(&mut self) -> Program {
        let mut items = Vec::new();

        while self.tokens.peek().is_some() {
            items.push(self.parse_item());
        }
        
        Program { items }
    }

    fn parse_item(&mut self) -> Item {
        if self.check(TokenVariant::Symbol(Symbol::Equals), 1) {
            Item::Definition(self.parse_definition())
        } else if self.check(TokenVariant::Symbol(Symbol::At), 0) {
            let sig = self.parse_function_type_decl();
            Item::Definition(Definition::Function(
                FunctionDef {
                    sig: Some(sig), 
                    head: self.parse_function_header(),
                }
            ))
        } else {
            Item::Expression(self.parse_expression())
        }
    }

    fn parse_definition(&mut self) -> Definition {

    }

    fn parse_function_type_decl(&mut self) -> FunctionTypeDecl {
        todo!()
    }

    fn parse_function_header(&mut self) -> FunctionHeader {
        todo!()
    }

    fn parse_expression(&mut self) -> Expression {
        todo!()
    }

    fn check(&mut self, expected: TokenVariant, n: usize) -> bool {
        match self.tokens.peek_n(n) {
            Some(c) => c.var == expected,
            None => false
        }
    }

    fn eat_if_match(&mut self, expected: TokenVariant) -> bool {
        if self.check(expected, 0) {
            self.tokens.eat();
            true
        } else {
            false
        }
    }
}
