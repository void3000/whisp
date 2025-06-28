use crate::runtime::evaluator::{ Evaluator, eval };
use crate::environment::Environment;
use crate::value::{ Value };

use whisp_parser::tree::ASTNode;
use whisp_parser::ops::Operation;

/// All the evaluator should is to walk through the AST and evaluate it. The AST
/// is a tree structure that represents the program. The evaluator will traverse
/// the tree and evaluate each node according to its type. The evaluator will
/// also handle the environment, which is a mapping of variable names to values.
pub struct Interpreter<'a> {
    env: &'a mut Environment,
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
    fn eval_str(&mut self, node: &ASTNode) -> Result<Value, String> {
        match node {
            ASTNode::Str { value } => Ok(Value::Str(value.clone())),
            _ => Err("Expected a string.".to_string()),
        }
    }

    fn eval_numeric(&mut self, node: &ASTNode) -> Result<Value, String> {
        match node {
            ASTNode::Numeric { value } => Ok(Value::Int(*value)),
            _ => Err("Expected a numeric.".to_string()),
        }
    }

    fn eval_boolean(&mut self, node: &ASTNode) -> Result<Value, String> {
        match node {
            ASTNode::Bool { value } => Ok(Value::Bool(*value)),
            _ => Err("Expected a boolean".to_string()),
        }
    }

    fn eval_array(&mut self, node: &ASTNode) -> Result<Value, String> {
        match node {
            ASTNode::Array { elements } => {
                let mut values = Vec::new();
                for elem in elements {
                    let value = eval(self, elem)?;
                    values.push(value);
                }
                Ok(Value::Array(values))
            }
            _ => Err("Expected an array.".to_string()),
        }
    }

    fn eval_array_index(&mut self, node: &ASTNode) -> Result<Value, String> {
        match node {
            ASTNode::ArrayIndex { arr, index } => {
                let array_list_eval = eval(self, arr)?;
                let index_eval      = eval(self, index)?;
                match (array_list_eval, index_eval) {
                    (Value::Array(arr), Value::Int(idx)) => {
                        let value = arr.get(idx as usize)
                            .ok_or_else(|| format!("Index {} out of bound.", idx))?
                            .clone();
                        Ok(value)
                    }
                    _ => Err("Expected a valid integer index.".to_string()),
                }
            }
            _ => Err("Expected an array index operation.".to_string()),
        }
    }

    fn eval_identifier(&mut self, node: &ASTNode) -> Result<Value, String> {
        match node {
            ASTNode::Identifier { name } => {
                self.env.get(name)
                    .ok_or_else(|| {
                        format!("Undeclared identifier '{}' referenced.", name)
                    })
            }
            _ => Err("Expected an identifier.".to_string()),
        }
    }

    fn eval_binary_op(
        &mut self,
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
            Operation::Mod => lhs_val.modulo(rhs_val),
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

#[cfg(test)]
mod test_inerpreter {
    use super::*;
    use crate::value::Value;
    use crate::environment::Environment;

    #[test]
    fn test_eval_numeric() {
        let mut env = Environment::new();
        let mut interpreter = Interpreter::new(&mut env);
        let ast = ASTNode::numeric(7);
        let result = eval(&mut interpreter, &ast);

        assert!(result.is_ok());
        match result {
            Ok(Value::Int(val)) => assert_eq!(val, 7),
            _ => panic!("Expected numeric value 7"),
        }
    }

    #[test]
    fn test_eval_array_index() {
        let mut env = Environment::new();
        let mut interpreter = Interpreter::new(&mut env);
        let ast = ASTNode::array_index(
            ASTNode::array(vec![
                ASTNode::numeric(1), 
                ASTNode::numeric(2), 
                ASTNode::numeric(3)
            ]),
            ASTNode::numeric(1)
        );
        let result = eval(&mut interpreter, &ast);

        assert!(result.is_ok());
        match result {
            Ok(Value::Int(val)) => assert_eq!(val, 2),
            _ => panic!("Expected numeric value 2"),
        }
    }
}