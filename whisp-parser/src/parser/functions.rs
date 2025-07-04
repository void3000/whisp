use crate::parser::ll_parser::{ Parser, LLParser };
use crate::symbol::{ SymbolTable, SymbolInfo };
use crate::tree::ASTNode;
use crate::ops::Operation;

use whisp_lexer::token::Token;
use whisp_lexer::lexer::TokenIterator;

impl<'a, T> LLParser<'a, T> 
where
    T: TokenIterator<Item = Token>,
{
    /// Function ::= 'def' Identifier '(' Params ')' Block
    pub fn parse_function_def(&mut self) -> Result<ASTNode, String> {
        self.expect(Token::Def);

        let identifier = self.parse_identifier()?;
        let ASTNode::Identifier { ref name } = identifier 
        else {
            return Err("Expected function name identifier after 'def'".to_string());
        };

        self.expect(Token::LParen);
        let params = self.parse_params()?;
        self.expect(Token::RParen);

        let body = self.parse_block()?;

        self.symbols.define(name.clone(), SymbolInfo);

        Ok(ASTNode::function_def(identifier, params, body))
    }

    /// Call ::= Identifier '(' Args ')'
    pub fn parse_call_function(&mut self) -> Result<ASTNode, String> {
        let identifier = self.parse_identifier()?;
        let ASTNode::Identifier { ref name } = identifier 
        else {
            return Err("Expected function identifier".to_string());
        };

        if self.symbols.resolve(name).is_none() {
            return Err(format!("Undefined function name found '{}'", name));
        }

        self.expect(Token::LParen);
        let args = self.parse_args()?;
        self.expect(Token::RParen);

        Ok(ASTNode::call(identifier, args))
    }

    /// Params ::= Identifier ParamsTrail | ε
    pub fn parse_params(&mut self) -> Result<Vec<ASTNode>, String> {
        let mut params = Vec::new();

        if self.peek() == &Token::RParen {
            return Ok(params);
        }

        params.push(self.parse_identifier()?);
        let _ = self.parse_params_trail(&mut params);

        Ok(params)
    }

    /// ParamsTrail ::= ',' Identifier ParamsTrail | ε
    pub fn parse_params_trail(
        &mut self,
        params: &mut Vec<ASTNode>
    ) -> Result<(), String> {
        while matches!(self.peek(), Token::Comma) {
            self.advance();
            params.push(self.parse_identifier()?);
        }
        Ok(())
    }

    /// Args ::= ArgTerm ArgsTail | ε
    pub fn parse_args(&mut self) -> Result<Vec<ASTNode>, String> {
        let mut args = Vec::new();

        if matches!(self.peek(), Token::RParen) {
            return Ok(args);
        }

        args.push(self.parse_arg_term()?);
        let _ = self.parse_args_trail(&mut args);

        Ok(args)
    }

    /// ArgTerm ::= Literal | Identifier
    pub fn parse_arg_term(&mut self) -> Result<ASTNode, String> {
        match self.peek() {
            Token::String(_)
            | Token::Int(_)
            | Token::Bool(_) => self.parse_literal(),
            Token::Identifier(_) => {
                let identifier = self.parse_identifier();

                let Ok(ASTNode::Identifier { ref name }) = identifier 
                else {
                    return Err(format!("Expected identifier as argument, got {:?}", identifier));
                };

                if self.symbols.resolve(name).is_none() {
                    return Err(format!("Undeclared variable '{}'", name));
                }

                identifier
            },
            _ => Err(format!("Expected argument term, found {:?}", self.peek())),
        }
    }

    /// ArgsTail ::= ',' ArgTerm ArgsTail | ε
    pub fn parse_args_trail(
        &mut self, 
        args: &mut Vec<ASTNode>
    ) -> Result<(), String> {
        while matches!(self.peek(), Token::Comma) {
            self.advance();
            args.push(self.parse_arg_term()?);
        }

        Ok(())
    }


}

#[cfg(test)]
mod test_functions {
    use super::*;
    use crate::parser::ll_parser::LLParser;
    use crate::mock::MockStream;
    use whisp_lexer::token::Token;

    #[test]
    fn test_function_def() {
        let tokens = vec![
            Token::Def,
            Token::Identifier("my_function".into()),
            Token::LParen,
            Token::Identifier("a".into()),
            Token::Comma,
            Token::Identifier("b".into()),
            Token::RParen,
            Token::LBrace,
            Token::Let,
            Token::Identifier("result".into()),
            Token::Assign,
            Token::Identifier("a".into()),
            Token::Add,
            Token::Identifier("b".into()),
            Token::Semicolon,
            Token::RBrace,
            Token::Eof
        ];
        let stream = MockStream::new(tokens);

        let mut symbols = SymbolTable::new();
        let mut parser = LLParser::new(stream, &mut symbols);

        let ast = parser.parse();

        assert!(ast.is_ok());

        match ast.unwrap() {
            ASTNode::Statements { stmts } => {
                assert_eq!(stmts.len(), 1);
                assert_eq!(stmts[0], ASTNode::function_def(
                    ASTNode::identifier("my_function"),
                    vec![ASTNode::identifier("a"), ASTNode::identifier("b")],
                    ASTNode::statements(vec![
                        ASTNode::let_binding(
                            ASTNode::identifier("result"),
                            ASTNode::binary_op(
                                Operation::Add,
                                ASTNode::identifier("a"),
                                ASTNode::identifier("b")
                            )
                        )
                    ])
                ));
            },
            _ => panic!("Expected valid statement."),
        }
    }

    #[test]
    fn test_call_function() {
        let tokens = vec![
            Token::Def,
            Token::Identifier("my_function".into()),
            Token::LParen,
            Token::Identifier("a".into()),
            Token::Comma,
            Token::Identifier("b".into()),
            Token::RParen,
            Token::LBrace,
            Token::Let,
            Token::Identifier("result".into()),
            Token::Assign,
            Token::Identifier("a".into()),
            Token::Add,
            Token::Identifier("b".into()),
            Token::Semicolon,
            Token::RBrace,
            Token::Let,
            Token::Identifier("a".into()),
            Token::Assign,
            Token::Int(5),
            Token::Semicolon,
            Token::Identifier("my_function".into()),
            Token::LParen,
            Token::Identifier("a".into()),
            Token::Comma,
            Token::Int(10),
            Token::RParen,
            Token::Semicolon,
            Token::Eof
        ];
        let stream = MockStream::new(tokens);

        let mut symbols = SymbolTable::new();
        let mut parser = LLParser::new(stream, &mut symbols);
        
        let ast = parser.parse();

        assert!(ast.is_ok());
        let expected_ast = ASTNode::statements(vec![
            ASTNode::function_def(
                ASTNode::identifier("my_function"),
                vec![
                    ASTNode::identifier("a"),
                    ASTNode::identifier("b"),
                ],
                ASTNode::statements(vec![
                    ASTNode::let_binding(
                        ASTNode::identifier("result"),
                        ASTNode::binary_op(
                            Operation::Add,
                            ASTNode::identifier("a"),
                            ASTNode::identifier("b"),
                        ),
                    ),
                ]),
            ),
            ASTNode::let_binding(
                ASTNode::identifier("a"),
                ASTNode::numeric(5),
            ),
            ASTNode::call(
                ASTNode::identifier("my_function"),
                vec![
                    ASTNode::identifier("a"),
                    ASTNode::numeric(10),
                ],
            ),
        ]);

        assert_eq!(ast.unwrap(), expected_ast);
    }

    #[test]
    fn test_call_undefined_function_then_fail() {
        let tokens = vec![
            Token::Let,
            Token::Identifier("a".into()),
            Token::Assign,
            Token::Int(5),
            Token::Semicolon,
            Token::Identifier("my_function".into()),
            Token::LParen,
            Token::Identifier("a".into()),
            Token::Comma,
            Token::Int(10),
            Token::RParen,
            Token::Semicolon,
            Token::Eof
        ];
        let stream = MockStream::new(tokens);
        let mut symbols = SymbolTable::new();

        let mut parser = LLParser::new(stream, &mut symbols);
        let ast = parser.parse();

        assert!(ast.is_err());
        
        let err = ast.unwrap_err();
        assert!(err.contains("Undefined function name found 'my_function'"));
    }
}
