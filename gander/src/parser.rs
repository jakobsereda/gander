use std::iter::Peekable;

use crate::{
    ast::*,
    token::Token,
    token::TokenVariant,
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
        todo!()
    }

    fn peek(&mut self) -> Option<&Token> {
        self.tokenizer.peek()
    }

    // Okay so... I don't like how this is different
    // from the other peek API. So now I am wondering
    // what I should do to make the lookahead system
    // better. AI is suggesting I create a wrapper type
    // around the tokenizer that allocates and maintains
    // a buffer of tokens... I'm not sure if I like that
    // and might take another look at refactoring the
    // tokenizer, but there is definitely more work to
    // be done in that regard.
    fn peek_two(&mut self) -> Option<Token> {
        let mut clone = self.tokenizer.clone();
        clone.next()?;
        clone.next()
    }

    fn eat(&mut self) -> Option<Token> {
        self.tokenizer.next()
    }

    fn check(&mut self, expected: TokenVariant) -> bool {
        match self.peek() {
            Some(c) => c.var == expected,
            None => false
        }
    }

    fn eat_if_match(&mut self, expected: TokenVariant) -> bool {
        if self.check(expected) {
            self.eat();
            true
        } else {
            false
        }
    }
}
