use crate::evaluator::{ Evaluator, eval };
use crate::environment::Environment;
use crate::value_type::{ Value };

use whisp_parser::ast::ASTNode;
use whisp_parser::ops::Operation;

/// All the evaluator should is to walk through the AST and evaluate it. The AST
/// is a tree structure that represents the program. The evaluator will traverse
/// the tree and evaluate each node according to its type. The evaluator will
/// also handle the environment, which is a mapping of variable names to values.
pub struct Interpreter<'a> {
    env: &'a Environment,
}

impl<'a> Interpreter<'a> {
    /// Example usage:
    /// 
    /// ```whisp
    /// let mut env = Environment::new();
    /// let interpreter = Interpreter::new(&mut env);
    /// let ast = ASTNode::Str { value: "hello".into() };
    ///
    /// eval(&interpreter, &ast);
    /// ```
    pub fn new(env: &'a mut Environment) -> Self {
        Interpreter { env }
    }
}

impl<'a> Evaluator for Interpreter<'a> {
    fn eval_str(&self, node: &ASTNode) -> Result<Value, String> {
        match node {
            ASTNode::Str { value } => Ok(Value::Str(value.clone())),
            _ => Err("Expected a string node".to_string()),
        }
    }

    fn eval_numeric(&self, node: &ASTNode) -> Result<Value, String> {
        match node {
            ASTNode::Numeric { value } => Ok(Value::Int(*value)),
            _ => Err("Expected a numeric node".to_string()),
        }
    }

    fn eval_boolean(&self, node: &ASTNode) -> Result<Value, String> {
        match node {
            ASTNode::Bool { value } => Ok(Value::Bool(*value)),
            _ => Err("Expected a boolean node".to_string()),
        }
    }

    fn eval_binary_op(
        &self,
        op: &Operation,
        lhs: &ASTNode,
        rhs: &ASTNode,
    ) -> Result<Value, String> {
        let lhs_val = eval(self, lhs)?;
        let rhs_val = eval(self, rhs)?;

        let result = match op {
            Operation::Add => lhs_val.add(rhs_val),
            Operation::Sub => lhs_val.sub(rhs_val),
            Operation::Mul => lhs_val.mul(rhs_val),
            Operation::Div => lhs_val.div(rhs_val),
            Operation::Eq  => lhs_val.eq(rhs_val),
            Operation::Lt  => lhs_val.lt(rhs_val),
            Operation::Gt  => lhs_val.gt(rhs_val),
            Operation::Le  => lhs_val.le(rhs_val),
            Operation::Ge  => lhs_val.ge(rhs_val),
            Operation::And => lhs_val.and(rhs_val),
            Operation::Or  => lhs_val.or(rhs_val),
        };

        Ok(result)
    }
}
