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
        let ASTNode::Str { value } = node 
        else {
            return Err("Expected a valid string.".to_string());
        };

        Ok(Value::Str(value.clone()))
    }

    fn eval_numeric(&mut self, node: &ASTNode) -> Result<Value, String> {
        let ASTNode::Numeric { value } = node 
        else {
            return Err("Expected a valid numeric.".to_string());
        };

        Ok(Value::Int(*value))
    }

    fn eval_boolean(&mut self, node: &ASTNode) -> Result<Value, String> {
        let ASTNode::Bool { value } = node 
        else {
            return Err("Expected a valid boolean.".to_string());
        };

        Ok(Value::Bool(*value))
    }

    fn eval_array(&mut self, node: &ASTNode) -> Result<Value, String> {
        let ASTNode::Array { elements } = node 
        else {
            return Err("Expected a valid array.".to_string());
        };
        let mut values = Vec::new();
        
        for elem in elements {
            let value = eval(self, elem)?;
            values.push(value);
        }
        
        Ok(Value::Array(values))
    }

    fn eval_array_index(&mut self, node: &ASTNode) -> Result<Value, String> {
        let ASTNode::ArrayIndex { 
            arr, 
            index 
        } = node 
        else {
            return Err("Expected a valid array index operation.".to_string());
        };

        let arr_eval = eval(self, arr)?;
        let index_eval = eval(self, index)?;
        
        match (arr_eval, index_eval) {
            (Value::Array(arr), Value::Int(idx)) => {
                let value = arr.get(idx as usize)
                        .ok_or_else(|| format!("Index {} out of bound.", idx))?
                        .clone();
                Ok(value)
            }
            _ => Err("Expected a valid integer index.".to_string()),
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

    fn eval_letbinding(&mut self, node: &ASTNode) -> Result<Value, String> {
        let ASTNode::LetBinding { 
            identifier, 
            body 
        } = node 
        else {
            return Err("Expected a valid variable binding operation.".to_string());
        };
        let ASTNode::Identifier { name } = &**identifier 
        else {
            return Err("Expected a valid identifier for variable binding.".to_string());
        };
    
        let eval_value = eval(self, body)?;
        self.env.put(name.clone(), eval_value);

        Ok(Value::Void(()))
    }

    fn eval_assgin(&mut self, node: &ASTNode) -> Result<Value, String> {
        let ASTNode::Assign { 
            identifier, 
            body 
        } = node 
        else {
            return Err("Expected a valid assignment operation.".to_string());
        };

        let ASTNode::Identifier { name } = &**identifier 
        else {
            return Err("Expected a valid identifier for assignment.".to_string());
        };

        let eval_value = eval(self, body)?;
        let result = self.env.update(&name.clone(), eval_value);
        
        match result {
            Ok( _ )  => Ok(Value::Void(())),
            Err(err) => Err(err),
        }
    }

    fn eval_statements(&mut self, node: &ASTNode) -> Result<Value, String> {
        let ASTNode::Statements { stmts } = node 
        else {
            return Err("Expected a valid sequence of statements.".to_string());
        };
        let mut last_value = Value::Void(());

        for stmt in stmts {
            last_value = eval(self, stmt)?;
        }

        Ok(last_value)

    }

    fn eval_whileloop(&mut self, node: &ASTNode) -> Result<Value, String> {
        let ASTNode::WhileLoop { 
            cond, 
            body 
        } = node 
        else {
            return Err("Expected a valid while loop.".to_string());
        };
        
        self.env.enter_scope();

        while matches!(eval(self, cond)?, Value::Bool(true)) {
            eval(self, body)?;
        }
        self.env.exit_scope();
    
        Ok(Value::Void(()))
    }

    fn eval_forloop(&mut self, node: &ASTNode) -> Result<Value, String> {
        let ASTNode::ForLoop { 
            itr, 
            var, 
            body 
        } = node 
        else {
            return Err("Expected a valid for-loop.".to_string());
        };

        let iterable = eval(self, itr)?;
        let Value::Array(elements) = iterable 
        else {
            return Err("Expected an iterable for the for-loop.".to_string());
        };

        let ASTNode::Identifier { name } = &**var 
        else {
            return Err("Expected a valid identifier for the for-loop.".to_string());
        };

        self.env.enter_scope();

        for item in elements {
            self.env.put(name.clone(), item);
            eval(self, body)?;
        }
        self.env.exit_scope();

        Ok(Value::Void(()))
    }

    fn eval_ifstatement(&mut self, node: &ASTNode) -> Result<Value, String> {
        let ASTNode::IfStatement { 
            cond, 
            then_branch, 
            else_branch 
        } = node 
        else {
            return Err("Expected a valid if statement.".to_string());
        };

        match eval(self, cond)? {
            Value::Bool(true) => {
                self.env.enter_scope();
                let result = eval(self, then_branch);
                self.env.exit_scope();
                result
            },
            Value::Bool(false) => {
                if let Some(else_branch) = else_branch {
                    self.env.enter_scope();
                    let result = eval(self, then_branch);
                    self.env.exit_scope();
                    result
                } else {
                    Ok(Value::Void(()))
                }
            }
            _ => Err("If statement condition must be a boolean expression.".to_string()),
        }
    }

    fn eval_function_def(&mut self, node: &ASTNode) -> Result<Value, String> {
        let ASTNode::FunctionDef {
            name,
            params,
            body
        } = node
        else {
            return Err("Expected a valid function definition.".to_string());
        };
        let ASTNode::Identifier { name } = &**name 
        else {
            return Err("Expected a valid identifier as function name.".to_string());
        };
        let all_valid_parameters = params
                .iter()
                .all(|p| matches!(p, ASTNode::Identifier { .. }));
        if !all_valid_parameters {
            return Err("All function parameters must be identifiers.".to_string());
        }
        let fun_defition = Value::Function {
            params: params.clone(),
            body: body.clone()
        };

        self.env.put(name.clone(), fun_defition);
        Ok(Value::Void(()))
    }

    fn eval_return(&mut self, node: &ASTNode) -> Result<Value, String> {
        match node {
            ASTNode::Return { value } => {
                let eval_value = eval(self, value)?;
                Ok(Value::Return(Box::new(eval_value)))
            },
            _ => Err("Expected a valid return statement.".to_string())
        }
    }

    fn eval_function_call(&mut self, node: &ASTNode) -> Result<Value, String> {
        let ASTNode::Call {
            name,
            args 
        } = node 
        else {
            return Err("Expected a valid function call.".to_string());
        };
        let eval_fun = eval(self, name)?;
        let eval_args: Vec<Value> = args
                .iter()
                .map(|arg| eval(self, arg).unwrap())
                .collect();

        match eval_fun {
            Value::Function { params, body } => {
                if eval_args.len() != params.len() {
                    return Err(format!(
                        "Expected {} parameters, but got {}", params.len(), eval_args.len()
                    ));
                }
                
                self.env.enter_scope();
                
                for (param, arg_val) in params.iter().zip(eval_args.into_iter()) {
                    let ASTNode::Identifier { name } = param 
                    else {
                        return Err("Function parameters must be identifiers.".to_string());
                    };
                    self.env.put(name.clone(), arg_val);
                }

                let result = eval(self, &body)?;

                self.env.exit_scope();
                match result {
                    Value::Return(inner) => Ok(*inner),
                    _ => Ok(Value::Void(())),
                }
            }
            _ => Err("Error encountred while evaluating function call.".to_string())
        }
    }
}


#[cfg(test)]
mod test_interpreter {
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
        assert_eq!(result.unwrap(), Value::Int(2));
    }

    #[test]
    fn test_eval_array_index_out_of_bound() {
        let mut env = Environment::new();
        let mut interpreter = Interpreter::new(&mut env);
        let ast = ASTNode::array_index(
            ASTNode::array(vec![
                ASTNode::numeric(1), 
                ASTNode::numeric(2), 
                ASTNode::numeric(3)
            ]),
            ASTNode::numeric(4)
        );
        let result = eval(&mut interpreter, &ast);

        assert!(result.is_err());

        let err = result.unwrap_err();
        assert!(err.contains("Index 4 out of bound."));
    }

    #[test]
    fn test_let_binding() {
        let mut env = Environment::new();
        let mut interpreter = Interpreter::new(&mut env);
        let ast = ASTNode::let_binding(
            ASTNode::identifier("x".to_string()),
            ASTNode::numeric(42)
        );
        let result = eval(&mut interpreter, &ast);

        assert!(result.is_ok());
        assert_eq!(env.get("x"), Some(Value::Int(42)));
    }

    #[test]
    fn test_assign_undeclared_variable() {
        let mut env = Environment::new();
        let mut interpreter = Interpreter::new(&mut env);
        let ast = ASTNode::assign(
            ASTNode::identifier("w".to_string()),
            ASTNode::numeric(42)
        );
        let result = eval(&mut interpreter, &ast);

        assert!(result.is_err());

        let err = result.unwrap_err();
        assert!(err.contains("Undeclared variable 'w' referenced."));
    }

    #[test]
    fn test_statements() {
        let mut env = Environment::new();
        let mut interpreter = Interpreter::new(&mut env);
        let ast = ASTNode::statements(vec![
            ASTNode::let_binding(
                ASTNode::identifier("a".to_string()),
                ASTNode::numeric(10)
            ),
            ASTNode::let_binding(
                ASTNode::identifier("b".to_string()),
                ASTNode::numeric(20)
            ),
            ASTNode::binary_op(
                Operation::Add,
                ASTNode::identifier("a".to_string()),
                ASTNode::identifier("b".to_string())
            ),
        ]);
        let result = eval(&mut interpreter, &ast);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Value::Int(30));
    }

    #[test]
    fn test_while_loop() {
        let mut env = Environment::new();
        let mut interpreter = Interpreter::new(&mut env);
        let ast = ASTNode::statements(vec![
            ASTNode::let_binding(
                ASTNode::identifier("x".to_string()),
                ASTNode::numeric(0)
            ),
            ASTNode::while_loop(
                ASTNode::binary_op(
                    Operation::Lt,
                    ASTNode::identifier("x".to_string()),
                    ASTNode::numeric(3)
                ),
                ASTNode::assign(
                    ASTNode::identifier("x".to_string()),
                    ASTNode::binary_op(
                        Operation::Add,
                        ASTNode::identifier("x".to_string()),
                        ASTNode::numeric(1)
                    )
                )
            ),
        ]);
    
        let result = eval(&mut interpreter, &ast);
        assert!(result.is_ok());
        assert_eq!(env.get("x"), Some(Value::Int(3)));
    }

    #[test]
    fn test_for_loop() {
        let mut env = Environment::new();
        let mut interpreter = Interpreter::new(&mut env);
        let ast = ASTNode::statements(vec![
            ASTNode::let_binding(
                ASTNode::identifier("sum".to_string()),
                ASTNode::numeric(0)
            ),
            ASTNode::for_loop(
                ASTNode::array(vec![
                    ASTNode::numeric(1),
                    ASTNode::numeric(2),
                    ASTNode::numeric(3),
                ]),
                ASTNode::identifier("num".to_string()),
                ASTNode::assign(
                    ASTNode::identifier("sum".to_string()),
                    ASTNode::binary_op(
                        Operation::Add,
                        ASTNode::identifier("sum".to_string()),
                        ASTNode::identifier("num".to_string())
                    )
                )
            ),
        ]);

        let result = eval(&mut interpreter, &ast);

        assert!(result.is_ok());
        assert_eq!(env.get("sum"), Some(Value::Int(6)));
    }

    #[test]
    fn test_if_statement() {
        let mut env = Environment::new();
        let mut interpreter = Interpreter::new(&mut env);
        let ast = ASTNode::statements(vec![
            ASTNode::let_binding(
                ASTNode::identifier("x".to_string()),
                ASTNode::numeric(0)
            ),
            ASTNode::if_statement(
                ASTNode::boolean(true),
                ASTNode::statements(vec![
                    ASTNode::assign(
                        ASTNode::identifier("x".to_string()),
                        ASTNode::numeric(7)
                    )
                ]),
                Some(ASTNode::statements(vec![
                    ASTNode::assign(
                        ASTNode::identifier("x".to_string()),
                        ASTNode::numeric(3)
                    )
                ]))
            )
        ]);

        let result = eval(&mut interpreter, &ast);
    
        assert!(result.is_ok());
        assert_eq!(env.get("x"), Some(Value::Int(7)));
    }

    #[test]
    fn test_function_call() {
        let mut env = Environment::new();
        let mut interpreter = Interpreter::new(&mut env);
        let ast = ASTNode::statements(vec![
            ASTNode::function_def(
                ASTNode::identifier("return_value"),
                vec![
                    ASTNode::identifier("value")
                ],
                ASTNode::return_stmt(
                    ASTNode::identifier("value")
                )
            ),
            ASTNode::call(
                ASTNode::identifier("return_value"),
                vec![
                    ASTNode::string("Hello world!")
                ],
            )
        ]);

        let result = eval(&mut interpreter, &ast);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Value::Str("Hello world!".to_string()));
    }

    #[test]
    fn test_variable_scoping() {
        let mut env = Environment::new();
        let mut interpreter = Interpreter::new(&mut env);
        let ast = ASTNode::statements(vec![
            ASTNode::let_binding(
                ASTNode::identifier("x".to_string()),
                ASTNode::numeric(5)
            ),
            ASTNode::let_binding(
                ASTNode::identifier("y".to_string()),
                ASTNode::numeric(3)
            ),
            ASTNode::statements(vec![
                ASTNode::let_binding(
                    ASTNode::identifier("x".to_string()),
                    ASTNode::numeric(7)
                ),
                ASTNode::assign(
                    ASTNode::identifier("y".to_string()),
                    ASTNode::binary_op(
                        Operation::Add,
                        ASTNode::identifier("y".to_string()),
                        ASTNode::identifier("x".to_string())
                    )
                )
            ]),
            ASTNode::identifier("y".to_string())
        ]);

        let result = eval(&mut interpreter, &ast);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Value::Int(10));
    }
}
