use super::types::*;

pub fn tokenize(input: &str) -> Result<Vec<Token>, TokenizerError> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut chars = input.chars().peekable();
    let mut row = 0;
    let mut col = 0;

    while let Some(&c) = chars.peek() {
        match c {
            '(' => {
                tokens.push(Token {
                    var: TokenVariant::Symbol(Symbol::LParen),
                    lit: String::from("("),
                    row,
                    col
                });
                chars.next();
            }
            ')' => {
                tokens.push(Token {
                    var: TokenVariant::Symbol(Symbol::RParen),
                    lit: String::from(")"),
                    row,
                    col
                });
                chars.next();
            }
            '@' => {
                tokens.push(Token {
                    var: TokenVariant::Symbol(Symbol::At),
                    lit: String::from("@"),
                    row,
                    col
                });
                chars.next();
            }
            '$' => {
                tokens.push(Token {
                    var: TokenVariant::Symbol(Symbol::Dollar),
                    lit: String::from("$"),
                    row,
                    col
                });
                chars.next();
            }
            '#' => {
                tokens.push(Token {
                    var: TokenVariant::Symbol(Symbol::Hash),
                    lit: String::from("#"),
                    row,
                    col
                });
                chars.next();
            }
            '+' => {
                tokens.push(Token {
                    var: TokenVariant::Symbol(Symbol::Plus),
                    lit: String::from("+"),
                    row,
                    col
                });
                chars.next();
            }
            '=' => {
                chars.next();
                match chars.peek() {
                    Some(&'=') => {
                        tokens.push(Token {
                            var: TokenVariant::Symbol(Symbol::DoubleEquals),
                            lit: String::from("=="),
                            row,
                            col
                        });
                        chars.next();
                    }
                    _ => {
                        tokens.push(Token {
                            var: TokenVariant::Symbol(Symbol::Equals),
                            lit: String::from("="),
                            row,
                            col
                        });
                    }
                }
            }
            _ => return Err(TokenizerError::Unknown(row))
        }

        row += 1;
    }

    Ok(tokens)
}