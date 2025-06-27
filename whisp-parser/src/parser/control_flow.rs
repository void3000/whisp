use crate::parser::ll_parser::{ Parser, LLParser };
use crate::tree::ASTNode;

use whisp_lexer::token::Token;

impl LLParser {
    /// ControlFlow ::= IfStatement | WhileStatement | ForStatement | Return
    pub fn parse_control_flow(&mut self) -> Result<ASTNode, String> {
        match self.peek() {
            Token::If       => self.parse_ifstatement(),
            // Token::For      => self.parse_forstatement(),
            // Token::While    => self.parse_whilestatement(),
            Token::Return   => self.parse_return(),
            _ => Err(format!("Unexpected token: {:?}", self.peek())),
        }
    }

    /// Return ::= 'return' Expr ';'
    pub fn parse_return(&mut self) -> Result<ASTNode, String> {
        self.expect(Token::Return);
        let expr = self.parse_expression()?;
        self.expect(Token::Semicolon);

        Ok(ASTNode::return_stmt(expr))
    }

    /// IfStatement ::= 'if' BoolExpr Block IfStatementTail
    pub fn parse_ifstatement(&mut self) -> Result<ASTNode, String> {
        self.expect(Token::If);
        let cond = self.parse_expression()?;
        let then_branch = self.parse_block()?;
        let else_branch = self.parse_ifstatement_trail()?;

        Ok(ASTNode::if_statement(cond, then_branch, else_branch))
    }
 
    /// IfStatementTail ::= 'elif' BoolExpr Block IfStatementTail 
    ///                   | ElseStatement | ε
    pub fn parse_ifstatement_trail(&mut self) -> Result<Option<ASTNode>, String> {
        match self.peek() {
            Token::Elif => {
                self.expect(Token::Elif);
                let cond = self.parse_expression()?;
                let then_branch = self.parse_block()?;
                let else_branch = self.parse_ifstatement_trail()?;

                Ok(Some(ASTNode::if_statement(cond, then_branch, else_branch)))
            },
            Token::Else => {
                self.expect(Token::Else);
                let else_branch = self.parse_block()?;
                Ok(Some(else_branch))
            },
            _ => Ok(None),
        }
    }
}

#[cfg(test)]
mod test_control_flow {
    use super::*;
    use whisp_lexer::token::Token;

    #[test]
    fn test_parse_if_statement() {
        let mut parser = LLParser::new(vec![
            Token::If,
            Token::Bool(true),
            Token::LBrace,
            Token::Return,
            Token::Int(7),
            Token::Semicolon,
            Token::RBrace,
            Token::Else,
            Token::LBrace,
            Token::Return,
            Token::Int(4),
            Token::Semicolon,
            Token::RBrace,
        ]);

        let result = parser.parse_control_flow();
        assert!(result.is_ok());

        let ast = result.unwrap();

        assert_eq!(ast, 
            ASTNode::if_statement(
                ASTNode::boolean(true),
                ASTNode::sequence(vec![
                    ASTNode::return_stmt(ASTNode::numeric(7))
                ]),
                Some(ASTNode::sequence(vec![
                    ASTNode::return_stmt(ASTNode::numeric(4))
                ]))
            )
        );
    }

    #[test]
    fn test_parse_return_statement() {
        let mut parser = LLParser::new(vec![
            Token::Return,
            Token::Int(42),
            Token::Semicolon,
        ]);

        let result = parser.parse_control_flow();

        assert!(result.is_ok());
        
        let ast = result.unwrap();

        assert_eq!(ast, ASTNode::return_stmt(ASTNode::numeric(42)));
    }
}
