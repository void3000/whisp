use crate::parser::ll_parser::{ Parser, LLParser };
use crate::tree::ASTNode;
use crate::ops::Operation;

use whisp_lexer::token::Token;

impl LLParser {
    /// Function ::= 'def' Identifier '(' Params ')' Block
    pub fn parse_function_def(&mut self) -> Result<ASTNode, String> {
        self.expect(Token::Def);
        let identifier = self.parse_identifier()?;

        self.expect(Token::LParen);
        let params = self.parse_params()?;
        self.expect(Token::RParen);

        let body = self.parse_block()?;

        Ok(ASTNode::function_def(identifier, params, body))
    }

    /// Call ::= Identifier '(' Args ')'
    pub fn parse_call_function(&mut self) -> Result<ASTNode, String> {
        let identifier = self.parse_identifier()?;
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
            Token::Identifier(_) => self.parse_identifier(),
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
            Token::RBrace
        ];

        let mut parser = LLParser::new(tokens);
        let ast = parser.parse_statements();

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
            Token::Identifier("my_function".into()),
            Token::LParen,
            Token::Int(5),
            Token::Comma,
            Token::Int(10),
            Token::RParen,
        ];
        let mut parser = LLParser::new(tokens);
        let ast = parser.parse_call_function();

        assert!(ast.is_ok());
        match ast.unwrap() {
            ASTNode::Call { name, args } => {
                assert_eq!(name, Box::new(ASTNode::identifier("my_function")));
                assert_eq!(args.len(), 2);
                assert_eq!(args[0], ASTNode::numeric(5));
            },
            _ => panic!("Expected a function call"),
        }
    }
}
