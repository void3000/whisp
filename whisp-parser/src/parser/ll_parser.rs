use whisp_lexer::token::Token;
use crate::tree::ASTNode;

pub trait Parser {
    fn peek(&self) -> &Token;
    fn lookahead(&self) -> &Token;
    fn expect(&mut self, expected: Token);
    fn advance(&mut self);
    fn parse(&mut self) -> Result<ASTNode, String>;
}

pub struct LLParser {
    stream: Vec<Token>,
    cursor: usize
}

impl LLParser {
    pub fn new(stream: Vec<Token>) -> Self {
        LLParser {
            stream,
            cursor: 0,
        }
    }
}

impl Parser for LLParser {
    fn peek(&self) -> &Token {
        self.stream
            .get(self.cursor)
            .unwrap_or(&Token::Eof)
    }

    fn lookahead(&self) -> &Token {
        let lookahead = self.cursor + 1;
 
        self.stream
            .get(lookahead)
            .unwrap_or(&Token::Eof)
    }

    fn advance(&mut self) {
        if self.cursor < self.stream.len() { 
            self.cursor = self.cursor + 1;
        }
    }

    fn expect(&mut self, expected: Token) {
        if expected == *self.peek() {
            self.advance();
        } else {
            panic!("Expected {:?}, but found {:?}", expected, self.peek());
        }
    }

    fn parse(&mut self) -> Result<ASTNode, String> {
        Ok(self.parse_statements()?)
    }
}
