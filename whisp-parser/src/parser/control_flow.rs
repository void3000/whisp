use crate::parser::ll_parser::{ Parser, LLParser };
use crate::symbol::{ SymbolTable, SymbolInfo };
use crate::tree::ASTNode;


use whisp_lexer::token::Token;
use whisp_lexer::lexer::TokenIterator;

impl<'a, T> LLParser<'a, T> 
where
    T: TokenIterator<Item = Token>,
{
    /// ControlFlow ::= IfStatement | WhileStatement | ForStatement | Return
    pub fn parse_control_flow(&mut self) -> Result<ASTNode, String> {
        match self.peek() {
            Token::If       => self.parse_ifstatement(),
            Token::For      => self.parse_forstatement(),
            Token::While    => self.parse_whilestatement(),
            Token::Return   => self.parse_return(),
            _ => Err(format!("Unexpected token: {:?}", self.peek())),
        }
    }

    /// Return ::= 'return' Expr ';'
    pub fn parse_return(&mut self) -> Result<ASTNode, String> {
        self.expect(Token::Return)?;
        let expr = self.parse_expression()?;
        self.expect(Token::Semicolon)?;

        Ok(ASTNode::return_stmt(expr))
    }

    /// IfStatement ::= 'if' BoolExpr Block IfStatementTail
    pub fn parse_ifstatement(&mut self) -> Result<ASTNode, String> {
        self.expect(Token::If)?;
        let cond = self.parse_bool_expr()?;
        let then_branch = self.parse_block()?;
        let else_branch = self.parse_ifstatement_trail()?;

        Ok(ASTNode::if_statement(cond, then_branch, else_branch))
    }
 
    /// IfStatementTail ::= 'elif' BoolExpr Block IfStatementTail 
    ///                   | ElseStatement | ε
    pub fn parse_ifstatement_trail(&mut self) -> Result<Option<ASTNode>, String> {
        match self.peek() {
            Token::Elif => {
                self.expect(Token::Elif)?;
                let cond = self.parse_bool_expr()?;
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

    /// WhileStatement ::= 'while' BoolExpr Block
    pub fn parse_whilestatement(&mut self) -> Result<ASTNode, String> {
        self.expect(Token::While)?;
        let cond = self.parse_bool_expr()?;
        let body = self.parse_block()?;

        Ok(ASTNode::while_loop(cond, body))
    }

    /// ForStatement ::= 'for' Identifier 'in' Array Block
    pub fn parse_forstatement(&mut self) -> Result<ASTNode, String> {
        self.expect(Token::For)?;

        let var = self.parse_identifier()?;
        if let ASTNode::Identifier { ref name } = var {
            self.symbols.define(name.clone(), SymbolInfo);
        } else {
            return Err("Expected identifier after 'for'".to_string());
        }

        self.expect(Token::In)?;

        let itr = match self.peek() {
            Token::Array => self.parse_array()?,
            Token::Identifier(_) => {
            let ident = self.parse_identifier()?;

            if let ASTNode::Identifier { ref name } = ident {
                if None == self.symbols.resolve(name) {
                    return Err(format!("Undefined variable '{}' used as iterator", name));
                }
            }

            ident
        }
            other => return Err(format!("Expected array or identifier after 'in', found {:?}", other)),
        };

        let body = self.parse_block()?;

        Ok(ASTNode::for_loop(var, itr, body))
    }
}

#[cfg(test)]
mod test_control_flow {
    use super::*;
    use whisp_lexer::token::Token;
    use crate::mock::MockStream;
    use crate::parser::ll_parser::LLParser;
    use crate::tree::{ ASTNode };
    use crate::symbol::SymbolTable;

    #[test]
    fn test_parse_if_statement() {
        let tokens = vec![
            Token::If,
            Token::LParen,
            Token::Bool(true),
            Token::RParen,
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
        ];
        let stream = MockStream::new(tokens);
        let mut symbols = SymbolTable::new();
        let mut parser = LLParser::new(stream, &mut symbols);

        let result = parser.parse_control_flow();
        assert!(result.is_ok());

        let ast = result.unwrap();

        assert_eq!(ast, 
            ASTNode::if_statement(
                ASTNode::boolean(true),
                ASTNode::statements(vec![
                    ASTNode::return_stmt(ASTNode::numeric(7))
                ]),
                Some(ASTNode::statements(vec![
                    ASTNode::return_stmt(ASTNode::numeric(4))
                ]))
            )
        );
    }

    #[test]
    fn test_parse_if_statement_fail_when_non_bool_expr() {
        let tokens = vec![
            Token::If,
            Token::Int(1),
            Token::Add,
            Token::Int(2),
            Token::LBrace,
            Token::Return,
            Token::Int(7),
            Token::Semicolon,
            Token::RBrace,
        ];
        let stream = MockStream::new(tokens);
        let mut symbols = SymbolTable::new();
        let mut parser = LLParser::new(stream, &mut symbols);

        let result = parser.parse_control_flow();

        let err = result.unwrap_err();
        assert!(err.contains("Expected boolean value, found Int(1)"));
    }

    #[test]
    fn test_parse_return_statement() {
        let tokens = vec![
            Token::Return,
            Token::Int(42),
            Token::Semicolon,
        ];
        let stream = MockStream::new(tokens);
        let mut symbols = SymbolTable::new();
        let mut parser = LLParser::new(stream, &mut symbols);

        let result = parser.parse_control_flow();
        assert!(result.is_ok());

        let ast = result.unwrap();
        assert_eq!(ast, ASTNode::return_stmt(ASTNode::numeric(42)));
    }

    #[test]
    fn test_parse_while_statement() {
        let tokens = vec![
            Token::While,
            Token::Bool(false),
            Token::LBrace,
            Token::Return,
            Token::Int(0),
            Token::Semicolon,
            Token::RBrace,
        ];
        let stream = MockStream::new(tokens);
        let mut symbols = SymbolTable::new();
        let mut parser = LLParser::new(stream, &mut symbols);

        let result = parser.parse_control_flow();
        assert!(result.is_ok());

        let ast = result.unwrap();
        assert_eq!(ast, 
            ASTNode::while_loop(
                ASTNode::boolean(false),
                ASTNode::statements(vec![
                    ASTNode::return_stmt(ASTNode::numeric(0))
                ])
            )
        );
    }

    #[test]
    fn test_parse_for_statement() {
        let tokens = vec![
            Token::For,
            Token::Identifier("i".into()),
            Token::In,
            Token::Array,
            Token::LBracket,
            Token::Int(7),
            Token::Comma,
            Token::Int(3),
            Token::RBracket,
            Token::LBrace,
            Token::Return,
            Token::Identifier("i".into()),
            Token::Semicolon,
            Token::RBrace,
        ];
        let stream = MockStream::new(tokens);
        let mut symbols = SymbolTable::new();
        let mut parser = LLParser::new(stream, &mut symbols);

        let result = parser.parse_control_flow();
        assert!(result.is_ok());

        let ast = result.unwrap();
        assert_eq!(ast,
            ASTNode::for_loop(
                ASTNode::identifier("i"),
                ASTNode::array(vec![
                    ASTNode::numeric(7),
                    ASTNode::numeric(3)
                ]),
                ASTNode::statements(vec![
                    ASTNode::return_stmt(ASTNode::identifier("i"))
                ])
            )
        );
    }
}
