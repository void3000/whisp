use crate::parser::ll_parser::{ Parser, LLParser };
use crate::tree::ASTNode;

use whisp_lexer::token::Token;

impl LLParser {
    /// Grammar:
    /// Stmts ::= Stmt Stmts | ε
    pub fn parse_statements(&mut self) -> Result<ASTNode, String> {
        let mut stmts = Vec::<ASTNode>::new();

        loop {
            match self.peek() {
                Token::RBrace | Token::Eof => break,
                _ => stmts.push(self.parse_statement()?),
            }
        }

        Ok(ASTNode::sequence(stmts))
    }

    /// Grammar:
    /// Stmt ::= Expr ';'
    pub fn parse_statement(&mut self) -> Result<ASTNode, String> {
        let expr = self.parse_expression()?;
        self.expect(Token::Semicolon);
        Ok(expr)
    }
}
