use crate::token::Token;

pub trait TokenIterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

pub trait Collection {
    type Iter;

    fn stream(&self) -> Self::Iter;
}

struct Stream<'a> {
    input: &'a str,
    cursor: usize
}

struct Lexer<'a> {
    input: &'a str
}

impl<'a> TokenIterator for Stream<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        Some(Token::Let)
    }
}

impl<'a> Collection for Lexer<'a> {
    type Iter = Stream<'a>;

    fn stream(&self) -> Self::Iter {
        Stream { 
            input: self.input,
            cursor: 0
        }
    }
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Self {
        Lexer { input: input }
    }
}


#[cfg(test)]
mod tokenizer_tests {
    use super::*;
    use crate::token::Token;

    #[test]
    fn test_stream() {
        let lexer = Lexer::new("let x = 4;");
        let mut stream = lexer.stream();
        let result = stream.next();

        match result {
            Some(token) => assert_eq!(token, Token::Let),
            _ => panic!("not equal")
        }
    }
}
