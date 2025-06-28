use crate::parser::ll_parser::{ Parser, LLParser };
use crate::tree::ASTNode;

use whisp_lexer::token::Token;

impl LLParser {
    /// Stmts ::= Stmt Stmts | ε
    pub fn parse_statements(&mut self) -> Result<ASTNode, String> {
        let mut stmts = Vec::<ASTNode>::new();

        loop {
            match self.peek() {
                Token::RBrace | Token::Eof => break,
                _ => stmts.push(self.parse_statement()?),
            }
        }

        Ok(ASTNode::statements(stmts))
    }

    /// Stmt ::= Expr ';' | LetBinding | FunctionDef | Block | ControlFlow
    pub fn parse_statement(&mut self) -> Result<ASTNode, String> {
        let result = match self.peek() {
            Token::If
            | Token::While 
            | Token::For 
            | Token::Return => self.parse_control_flow(),
            Token::Def      => self.parse_function_def(),
            Token::Let      => self.parse_letbinding(),
            Token::LBrace   => self.parse_block(),
            _ => { 
                let expr = self.parse_expression()?;
                self.expect(Token::Semicolon);
                Ok(expr)
            }
        };

        result
    }

    /// LetBinding ::= 'let' Identifier '=' Expr ';'
    pub fn parse_letbinding(&mut self) -> Result<ASTNode, String> {
        self.expect(Token::Let);
        let identifier = self.parse_identifier()?;

        self.expect(Token::Assign);
        let body = self.parse_expression()?;
        self.expect(Token::Semicolon);

        Ok(ASTNode::let_binding(identifier, body))
    }

    /// Block ::= '{' Stmts '}'
    pub fn parse_block(&mut self) -> Result<ASTNode, String> {
        self.expect(Token::LBrace);
        let stmts = self.parse_statements()?;
        self.expect(Token::RBrace);

        Ok(stmts)
    }
}

#[cfg(test)]
mod test_statements {
    use super::*;
    use crate::parser::ll_parser::LLParser;
    use whisp_lexer::token::Token;

    #[test]
    fn test_parse_letbinding_statements() {
        let tokens = vec![
            Token::Let,
            Token::Identifier("x".into()),
            Token::Assign,
            Token::Int(42),
            Token::Semicolon,
            Token::Let,
            Token::Identifier("y".into()),
            Token::Assign,
            Token::Int(100),
            Token::Semicolon
        ];

        let mut parser = LLParser::new(tokens);
        let ast = parser.parse_statements();

        assert!(ast.is_ok());

        match ast.unwrap() {
            ASTNode::Statements { stmts } => {
                assert_eq!(stmts.len(), 2);
                assert_eq!(stmts[0], ASTNode::let_binding(
                    ASTNode::identifier("x"), 
                    ASTNode::numeric(42)
                ));
                assert_eq!(stmts[1], ASTNode::let_binding(
                    ASTNode::identifier("y"), 
                    ASTNode::numeric(100)
                ));
            },
            _ => panic!("Expected valid statement."),
        }
    }

    #[test]
    fn test_block_statements() {
        let tokens = vec![
            Token::LBrace,
            Token::Let,
            Token::Identifier("x".into()),
            Token::Assign,
            Token::Int(42),
            Token::Semicolon,
            Token::RBrace
        ];

        let mut parser = LLParser::new(tokens);
        let ast = parser.parse_statements();

        assert!(ast.is_ok());

        match ast.unwrap() {
            ASTNode::Statements { stmts } => {
                assert_eq!(stmts.len(), 1);
                assert_eq!(stmts[0], ASTNode::statements(
                    vec![ASTNode::let_binding(
                            ASTNode::identifier("x"), 
                            ASTNode::numeric(42)
                        )
                    ]
                ));
            },
            _ => panic!("Expected valid statement."),
        }
    }
}
