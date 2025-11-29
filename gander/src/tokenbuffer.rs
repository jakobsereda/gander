use crate::{token::Token, tokenizer::Tokenizer};

#[derive(Debug)]
pub struct TokenBuffer<'a> {
    tokenizer: Tokenizer<'a>,
    buf: Vec<Token>,
}

impl<'a> TokenBuffer<'a> {
    pub fn new(tokenizer: Tokenizer<'a>) -> Self {
        Self {
            tokenizer,
            buf: Vec::new(),
        }
    }

    pub fn peek_n(&mut self, n: usize) -> Option<&Token> {
        self.fill(n + 1);
        self.buf.get(n)
    }

    pub fn peek(&mut self) -> Option<&Token> {
        self.peek_n(0)
    }

    pub fn eat(&mut self) -> Option<Token> {
        self.fill(1);
        if self.buf.is_empty() {
            None
        } else {
            Some(self.buf.remove(0))
        }
    }

    fn fill(&mut self, n: usize) {
        while self.buf.len() < n {
            match self.tokenizer.next() {
                Some(t) => self.buf.push(t),
                None => break,
            }
        }
    }
}
