#[derive(Clone, Debug)]
pub struct Scanner<'a> {
    src: &'a str,
    pos: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            src,
            pos: 0,
        }
    }

    pub fn peek(&self) -> Option<char> {
        self.src[self.pos..].chars().next()
    }

    pub fn eat(&mut self) -> Option<char> {
        let c = self.peek()?;
        self.pos += c.len_utf8();
        Some(c)
    }
}
