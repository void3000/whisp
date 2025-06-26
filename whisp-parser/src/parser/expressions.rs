use crate::parser::ll_parser::{ Parser, LLParser };
use crate::tree::ASTNode;
use crate::ops::Operation;

use whisp_lexer::token::Token;

impl LLParser {
    /// Expr ::= AssignmentExpr
    pub fn parse_expression(&mut self) -> Result<ASTNode, String> {
        self.parse_assignment_expr()
    }

    /// AssignmentExpr ::= Identifier '=' Expr | ArithmeticExpr
    pub fn parse_assignment_expr(&mut self) -> Result<ASTNode, String> {
        if matches!(self.peek(), Token::Identifier(_)) &&
           matches!(self.lookahead(), Token::Assign) 
        {
            let identifier = self.parse_identifier()?;
            self.advance();
            let body = self.parse_expression()?;
            return Ok(ASTNode::assign(identifier, body));
        }

        self.parse_arithmetic_expr()
    }

    /// ArithmeticExpr ::= OrExpr
    pub fn parse_arithmetic_expr(&mut self) -> Result<ASTNode, String> {
        self.parse_or_expr()
    }

    /// OrExpr ::= AndExpr OrExprTail
    pub fn parse_or_expr(&mut self) -> Result<ASTNode, String> {
        let lhs = self.parse_and_expr()?;
        self.parse_or_expr_tail(lhs)
    }

    /// OrExprTail ::= 'or' AndExpr OrExprTail | ε
    pub fn parse_or_expr_tail(
        &mut self,
        mut lhs: ASTNode
    ) -> Result<ASTNode, String> {
        while matches!(self.peek(), Token::Or) {
            self.advance();
            let rhs = self.parse_and_expr()?;
            lhs = ASTNode::binary_op(Operation::Or, lhs, rhs);
        }

        Ok(lhs)
    }

    /// AndExpr ::= CompExpr AndExprTail
    pub fn parse_and_expr(&mut self) -> Result<ASTNode, String> {
        let lhs = self.parse_comp_expr()?;
        self.parse_and_expr_tail(lhs)
    }

    /// AndExprTail ::= 'and' CompExpr AndExprTail | ε
    pub fn parse_and_expr_tail(
        &mut self, 
        mut lhs: ASTNode
    ) -> Result<ASTNode, String> {
        while matches!(self.peek(), Token::And) {
            self.advance();
            let rhs = self.parse_comp_expr()?;
            lhs = ASTNode::binary_op(Operation::And, lhs, rhs);
        }

        Ok(lhs)
    }

    /// CompExpr ::= AddSubExpr CompExprTail
    pub fn parse_comp_expr(&mut self) -> Result<ASTNode, String> {
        let lhs = self.parse_add_sub_expr()?;
        self.parse_comp_expr_tail(lhs)
    }

    /// CompExprTail ::= ('==' | '<' | '>' | '<=' | '>=') AddSubExpr CompExprTail 
    ///                | ε
    pub fn parse_comp_expr_tail(
        &mut self, 
        mut lhs: ASTNode
    ) -> Result<ASTNode, String> {
        while matches!(self.peek(), 
            Token::Equal 
            | Token::GreaterEqual 
            | Token::GreaterThan 
            | Token::LessThan
            | Token::LessEqual
        ) {
            let op = match self.peek() {
                Token::Equal => Operation::Eq,
                Token::GreaterEqual => Operation::Ge,
                Token::GreaterThan  => Operation::Gt,
                Token::LessThan     => Operation::Le,
                Token::LessEqual    => Operation::Lt,
                _ => unreachable!(),
            };

            self.advance();
            let rhs = self.parse_add_sub_expr()?;
            lhs = ASTNode::binary_op(op, lhs, rhs);
        }

        Ok(lhs)
    }

    /// AddSubExpr ::= MulDivExpr AddSubExprTail
    pub fn parse_add_sub_expr(&mut self) -> Result<ASTNode, String> {
        let lhs = self.parse_mul_div_expr()?;
        self.parse_add_sub_expr_tail(lhs)
    }

    /// AddSubExprTail ::= ('+' | '-') MulDivExpr AddSubExprTail | ε
    pub fn parse_add_sub_expr_tail(
        &mut self, 
        mut lhs: ASTNode
    ) -> Result<ASTNode, String> {
        while matches!(self.peek(), Token::Add | Token::Sub) {
            let op = match self.peek() {
                Token::Add => Operation::Add,
                Token::Sub => Operation::Sub,
                _ => unreachable!(),
            };

            self.advance();
            let rhs = self.parse_mul_div_expr()?;
            lhs = ASTNode::binary_op(op, lhs, rhs);
        }

        Ok(lhs)
    }

    /// MulDivExpr ::= PrimaryExpr MulDivExprTail 
    pub fn parse_mul_div_expr(&mut self) -> Result<ASTNode, String> {
        let lhs = self.parse_primary_expr()?;
        self.parse_mul_div_expr_tail(lhs)
    }

    /// MulDivExprTail ::= ('*' | '/' | '%') PrimaryExpr MulDivExprTrail | ε
    pub fn parse_mul_div_expr_tail(
        &mut self, 
        mut lhs: ASTNode
    ) -> Result<ASTNode, String> {
        while matches!(self.peek(), Token::Mul | Token::Div | Token::Mod) {
            let op = match self.peek() {
                Token::Mul => Operation::Mul,
                Token::Div => Operation::Div,
                Token::Mod => Operation::Mod,
                _ => unreachable!(),
            };

            self.advance();
            let rhs = self.parse_primary_expr()?;
            lhs = ASTNode::binary_op(op, lhs, rhs);
        }

        Ok(lhs)
    }

    /// PrimaryExpr ::= Literal | Identifier | Call | ArrayIndex | '(' Expr ')'
    pub fn parse_primary_expr(&mut self) -> Result<ASTNode, String> {
        match self.peek() { 
            Token::Identifier(_) => {
                match self.lookahead() {
                    Token::LParen => self.parse_call_function(),
                    Token::LBracket => self.parse_array_index(),
                    _ => self.parse_identifier(),
                }
            },
            Token::LParen => {
                self.advance();
                let expr = self.parse_expression()?;
                self.expect(Token::RParen);
                Ok(expr)
            },
            Token::String(_)
            | Token::Int(_) 
            | Token::Bool(_) => self.parse_literal(),
            _ => Err(format!("Unexpected token: {:?}", self.peek())),
        }
    }

    /// ArrayIndex ::= Identifier '[' (Int | Identifier) ']'
    pub fn parse_array_index(&mut self) -> Result<ASTNode, String> {
        let identifier = self.parse_identifier()?;
        self.expect(Token::LBracket);

        let result = match self.peek() {
            Token::Int(index) => 
                Ok(ASTNode::array_index(identifier, ASTNode::numeric(*index))),
            Token::Identifier(name) => 
                Ok(ASTNode::array_index(identifier, ASTNode::identifier(name))),
            _ => Err(format!("Expected Int or Identifier for array index, found {:?}", self.peek())),
        };

        if result.is_ok() {
            self.advance();
            self.expect(Token::RBracket);
        }

        result
    }

    /// Call ::= Identifier '(' Args ')'
    pub fn parse_call_function(&mut self) -> Result<ASTNode, String> {
        let identifier = self.parse_identifier()?;
        self.expect(Token::LParen);

        let args = self.parse_args()?;
        self.expect(Token::RParen);

        Ok(ASTNode::call(identifier, args))
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
        while let Token::Comma = self.peek() {
            self.advance();
            args.push(self.parse_arg_term()?);
        }

        Ok(())
    }

    /// Literal ::= Int | String | Bool | Array
    pub fn parse_literal(&mut self) -> Result<ASTNode, String> {
        let result = match self.peek() {
            Token::Int(value)    => Ok(ASTNode::numeric(*value)),
            Token::String(value) => Ok(ASTNode::string(value)),
            Token::Bool(value)   => Ok(ASTNode::boolean(*value)),
            _ => Err(format!("Expected literal, found {:?}", self.peek())),
        };

        if result.is_ok() {
            self.advance();
        }

        result
    }

    pub fn parse_identifier(&mut self) -> Result<ASTNode, String> {
        match self.peek() {
            Token::Identifier(name) => {
                let identifier = ASTNode::identifier(name);
                self.advance();
                Ok(identifier)
            },
            _ => Err(format!("Expected identifier, found {:?}", self.peek())),
        }
    }
}


#[cfg(test)]
mod test_expressions {
    use super::*;
    use whisp_lexer::token::Token;

    #[test]
    fn test_parse_assignment_expr() {
        let mut parser = LLParser::new(vec![
            Token::Identifier("x".into()),
            Token::Assign,
            Token::Int(42),
            Token::Semicolon,
        ]);

        let result = parser.parse_expression();

        match result {
            Ok(ast) => {
                assert_eq!(ast, ASTNode::assign(
                    ASTNode::identifier("x"),
                    ASTNode::numeric(42)
                ));
            },
            Err(e) => panic!("Failed to parse expression: {}", e),
        }
    }

    #[test]
    fn test_parse_arithmetic_expr() {
        let mut parser = LLParser::new(vec![
            Token::Int(5),
            Token::Add,
            Token::Int(3),
            Token::Mul,
            Token::Int(2),
            Token::Semicolon,
        ]);
        let result = parser.parse_expression();

        match result {
            Ok(ast) => {
                assert_eq!(ast, ASTNode::binary_op(
                    Operation::Add,
                    ASTNode::numeric(5),
                    ASTNode::binary_op(
                        Operation::Mul,
                        ASTNode::numeric(3),
                        ASTNode::numeric(2)
                    )
                ));
            },
            Err(e) => panic!("Failed to parse expression: {}", e),
        }
    }

    #[test]
    fn test_parse_identifier() {
        let mut parser = LLParser::new(vec![Token::Identifier("y".into())]);
        let ast = parser.parse_identifier().unwrap();

        assert_eq!(ast, ASTNode::identifier("y"));
    }

    #[test]
    fn test_parse_literal() {
        let mut parser = LLParser::new(vec![
            Token::Int(10),
            Token::String("hello".into()),
            Token::Bool(true),
        ]);

        let int_ast = parser.parse_literal().unwrap();
        assert_eq!(int_ast, ASTNode::numeric(10));

        let string_ast = parser.parse_literal().unwrap();
        assert_eq!(string_ast, ASTNode::string("hello"));

        let bool_ast = parser.parse_literal().unwrap();
        assert_eq!(bool_ast, ASTNode::boolean(true));
    }
}
