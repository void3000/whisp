use whisp_lexer::token::Token;
use whisp_lexer::lexer::TokenIterator;

pub struct MockStream {
    pub tokens: Vec<Token>,
    pub cursor: usize,
}

impl MockStream {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            cursor: 0,
        }
    }
}

impl TokenIterator for MockStream {
    type Item = Token;

    fn next(&mut self) -> Result<Self::Item, String> {
        if self.cursor < self.tokens.len() {
            let token = self.tokens[self.cursor].clone();
            self.cursor += 1;
            Ok(token)
        } else {
            Ok(Token::Eof)
        }
    }
}
