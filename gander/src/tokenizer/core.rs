use super::types::*;

pub fn tokenize(input: &str) -> Result<Vec<Token>, TokenizerError> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut chars = input.chars().peekable();
    let mut line = 0;

    while let Some(&c) = chars.peek() {
        match c {
            _ => return Err(TokenizerError::Unknown(line))
        }
    }

    Ok(tokens)
}