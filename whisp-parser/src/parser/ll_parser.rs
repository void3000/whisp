use whisp_lexer::lexer::TokenIterator;
use whisp_lexer::token::Token;

use crate::tree::ASTNode;
use crate::symbol::SymbolTable;

pub trait Parser {
    fn peek(&self) -> &Token;
    fn lookahead(&self) -> &Token;
    fn expect(&mut self, expected: Token) -> Result<(), String>;
    fn advance(&mut self);
    fn parse(&mut self) -> Result<ASTNode, String>;
}

pub struct LLParser<'a, T>
where
    T: TokenIterator<Item = Token>,
{
    pub stream: T,
    pub lookahead: Option<Token>,
    pub next_token: Option<Token>,
    pub symbols: &'a mut SymbolTable
}


impl<'a, T> LLParser<'a, T>
where
    T: TokenIterator<Item = Token>,
{
    pub fn new(mut stream: T, symbols: &'a mut SymbolTable) -> Self {
        let lookahead = match stream.next() {
            Ok(token) => Some(token),
            Err( _ ) => Some(Token::Eof)
        };

        let next_token = match stream.next() {
            Ok(token) => Some(token),
            Err( _ ) => Some(Token::Eof)
        };

        Self {
            stream,
            lookahead,
            next_token,
            symbols,
        }
    }

    fn shift(&mut self) {
        self.lookahead = self.next_token.take();
        self.next_token = Some(match self.stream.next() {
                Ok(token) => token,
                Err( _ ) => Token::Eof
            });
    }
}

impl<'a, T> Parser for LLParser<'a, T>
where
    T: TokenIterator<Item = Token>,
{
    fn peek(&self) -> &Token {
        self.lookahead.as_ref().unwrap_or(&Token::Eof)
    }

    fn lookahead(&self) -> &Token {
        self.next_token.as_ref().unwrap_or(&Token::Eof)
    }

    fn advance(&mut self) {
        self.shift();
    }

    fn expect(&mut self, expected: Token) -> Result<(), String> {
        if *self.peek() == expected {
            self.advance();
            Ok(())
        } else {
            Err(format!("expected {:#?}, but found {:#?}", expected, self.peek()))
        }
    }

    fn parse(&mut self) -> Result<ASTNode, String> {
        Ok(self.parse_statements()?)
    }
}
