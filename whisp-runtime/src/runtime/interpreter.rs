use std::rc::Rc;
use std::cell::RefCell;

use crate::runtime::evaluator::{ Evaluator, eval };
use crate::environment::Environment;
use crate::object::{ WhispObj };

use whisp_parser::tree::ASTNode;
use whisp_parser::ops::Operation;

/// All the evaluator should is to walk through the AST and evaluate it. The AST
/// is a tree structure that represents the program. The evaluator will traverse
/// the tree and evaluate each node according to its type. The evaluator will
/// also handle the environment, which is a mapping of variable names to values.
pub struct Interpreter {
   pub env: Rc<RefCell<Environment>>,
}

impl Interpreter {
    /// ```whisp
    /// let env = Rc::new(RefCell::new(Environment::new()));
    /// let interpreter = Interpreter::new(Rc::clone(&env));
    /// let ast = ASTNode::Str { value: "hello".into() };
    ///
    /// eval(&interpreter, &ast);
    /// ```
    pub fn new(env: Rc<RefCell<Environment>>) -> Self {
        Interpreter { env }
    }

    pub fn lookup(&self, name: &str) -> Option<WhispObj> {
        self.env.borrow_mut().get(name)
    }

    pub fn register(&mut self, key: String, value: WhispObj) {
        self.env.borrow_mut().put(key, value)
    }

    pub fn update(&mut self, key: &str, value: WhispObj) -> Result<(), String> {
        self.env.borrow_mut().update(key, value)
    }

    fn enter_scope(&self) {
        self.env.borrow_mut().enter_scope()
    }

    fn exit_scope(&self) {
        self.env.borrow_mut().exit_scope()
    }
}

impl Evaluator for Interpreter {
    fn eval_str(&mut self, node: &ASTNode) -> Result<WhispObj, String> {
        let ASTNode::Str { value } = node 
        else {
            return Err("Expected a valid string.".to_string());
        };

        Ok(WhispObj::Str(value.clone()))
    }

    fn eval_numeric(&mut self, node: &ASTNode) -> Result<WhispObj, String> {
        let ASTNode::Numeric { value } = node 
        else {
            return Err("Expected a valid numeric.".to_string());
        };

        Ok(WhispObj::Int(*value))
    }

    fn eval_boolean(&mut self, node: &ASTNode) -> Result<WhispObj, String> {
        let ASTNode::Bool { value } = node 
        else {
            return Err("Expected a valid boolean.".to_string());
        };

        Ok(WhispObj::Bool(*value))
    }

    fn eval_array(&mut self, node: &ASTNode) -> Result<WhispObj, String> {
        let ASTNode::Array { elements } = node 
        else {
            return Err("Expected a valid array.".to_string());
        };
        let mut values = Vec::new();
        
        for elem in elements {
            let value = eval(self, elem)?;
            values.push(value);
        }
        
        Ok(WhispObj::Array(values))
    }

    fn eval_array_index(&mut self, node: &ASTNode) -> Result<WhispObj, String> {
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
            (WhispObj::Array(arr), WhispObj::Int(idx)) => {
                let value = arr.get(idx as usize)
                        .ok_or_else(|| format!("Index {} out of bound.", idx))?
                        .clone();
                Ok(value)
            }
            _ => Err("Expected a valid integer index.".to_string()),
        }
    }

    fn eval_identifier(&mut self, node: &ASTNode) -> Result<WhispObj, String> {
        match node {
            ASTNode::Identifier { name } => {
                self.lookup(name)
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
    ) -> Result<WhispObj, String> {
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

    fn eval_letbinding(&mut self, node: &ASTNode) -> Result<WhispObj, String> {
        let ASTNode::LetBinding { 
            identifier, 
            body 
        } = node 
        else {
            return Err("Expected a valid variable binding operation.".to_string());
        };
        let ASTNode::Identifier { name } = identifier.as_ref() 
        else {
            return Err("Expected a valid identifier for variable binding.".to_string());
        };
    
        let eval_value = eval(self, body)?;
        self.register(name.clone(), eval_value);

        Ok(WhispObj::Void(()))
    }

    fn eval_assgin(&mut self, node: &ASTNode) -> Result<WhispObj, String> {
        let ASTNode::Assign { 
            identifier, 
            body 
        } = node 
        else {
            return Err("Expected a valid assignment operation.".to_string());
        };

        let eval_value = eval(self, body)?;

        match identifier.as_ref() {
            ASTNode::Identifier { name } => {
                self.update(&name, eval_value.clone())?;
            }
            ASTNode::ArrayIndex { arr, index } => { 
                let array_val = eval(self, arr)?;
                let index_val = eval(self, index)?;

                match (array_val, index_val) {
                    (WhispObj::Array(mut vec), WhispObj::Int(i)) => {
                        let i = i as usize;
                        if i >= vec.len() {
                            return Err("Index out of bounds".to_string());
                        }

                        vec[i] = eval_value.clone();

                        match arr.as_ref() {
                            ASTNode::Identifier { name } => {
                                self.update(name, WhispObj::Array(vec))?;
                            }
                            _ => return Err("Only assignment to direct array variables is supported".to_string()),
                        }
                    }
                    _ => return Err("Invalid array assignment".to_string()),
                }
            }
            _ => return Err("Expected a valid identifier for assignment.".to_string()),
        }

        Ok(eval_value)
    }

    fn eval_statements(&mut self, node: &ASTNode) -> Result<WhispObj, String> {
        let ASTNode::Statements { stmts } = node 
        else {
            return Err("Expected a valid sequence of statements.".to_string());
        };
        let mut last_value = WhispObj::Void(());

        for stmt in stmts {
            last_value = eval(self, stmt)?;
        }

        Ok(last_value)

    }

    fn eval_whileloop(&mut self, node: &ASTNode) -> Result<WhispObj, String> {
        let ASTNode::WhileLoop { 
            cond, 
            body 
        } = node 
        else {
            return Err("Expected a valid while loop.".to_string());
        };
        
        self.enter_scope();

        while matches!(eval(self, cond)?, WhispObj::Bool(true)) {
            eval(self, body)?;
        }

        self.exit_scope();
    
        Ok(WhispObj::Void(()))
    }

    fn eval_forloop(&mut self, node: &ASTNode) -> Result<WhispObj, String> {
        let ASTNode::ForLoop { 
            itr, 
            var, 
            body 
        } = node 
        else {
            return Err("Expected a valid for-loop.".to_string());
        };

        let iterable = eval(self, itr)?;
        let WhispObj::Array(elements) = iterable 
        else {
            return Err("Expected an iterable for the for-loop.".to_string());
        };

        let ASTNode::Identifier { name } = &**var 
        else {
            return Err("Expected a valid identifier for the for-loop.".to_string());
        };

        self.enter_scope();
        for item in elements {
            self.register(name.clone(), item);
            eval(self, body)?;
        }
        self.exit_scope();

        Ok(WhispObj::Void(()))
    }

    fn eval_ifstatement(&mut self, node: &ASTNode) -> Result<WhispObj, String> {
        let ASTNode::IfStatement { 
            cond,
            then_branch,
            else_branch 
        } = node 
        else {
            return Err("Expected a valid if statement.".to_string());
        };

        match eval(self, cond)? {
            WhispObj::Bool(true) => {
                self.enter_scope();
                let result = eval(self, then_branch);
                self.exit_scope();
                result
            },
            WhispObj::Bool(false) => {
                if let Some(else_branch) = else_branch {
                    self.enter_scope();
                    let result = eval(self, else_branch);
                    self.exit_scope();
                    result
                } else {
                    Ok(WhispObj::Void(()))
                }
            }
            _ => Err("If statement condition must be a boolean expression.".to_string()),
        }
    }

    fn eval_function_def(&mut self, node: &ASTNode) -> Result<WhispObj, String> {
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
        let fun_defition = WhispObj::Function {
            params: params.clone(),
            body: body.clone()
        };

        self.register(name.clone(), fun_defition);
        Ok(WhispObj::Void(()))
    }

    fn eval_return(&mut self, node: &ASTNode) -> Result<WhispObj, String> {
        match node {
            ASTNode::Return { value } => {
                let eval_value = eval(self, value)?;
                Ok(WhispObj::Return(Box::new(eval_value)))
            },
            _ => Err("Expected a valid return statement.".to_string())
        }
    }

    fn eval_function_call(&mut self, node: &ASTNode) -> Result<WhispObj, String> {
        let ASTNode::Call {
            name,
            args 
        } = node 
        else {
            return Err("Expected a valid function call.".to_string());
        };
        let eval_fun = eval(self, name)?;
        let eval_args: Vec<WhispObj> = args
                .iter()
                .map(|arg| eval(self, arg).unwrap())
                .collect();

        match eval_fun {
            WhispObj::Function { params, body } => {
                if eval_args.len() != params.len() {
                    return Err(format!(
                        "Expected {} parameters, but got {}", params.len(), eval_args.len()
                    ));
                }
                
                self.enter_scope();
                
                for (param, arg_val) in params.iter().zip(eval_args.into_iter()) {
                    let ASTNode::Identifier { name } = param 
                    else {
                        return Err("Function parameters must be identifiers.".to_string());
                    };
                    self.register(name.clone(), arg_val);
                }

                let result = eval(self, &body)?;

                self.exit_scope();
                match result {
                    WhispObj::Return(inner) => Ok(*inner),
                    _ => Ok(WhispObj::Void(())),
                }
            },
            WhispObj::BuiltInFunction { callback } => {
                Ok(callback.call(eval_args))
            },
            _ => Err("Error encountred while evaluating function call.".to_string())
        }
    }
}


// #[cfg(test)]
// mod test_interpreter {
//     use super::*;
//     use crate::object::WhispObj;
//     use crate::environment::Environment;

//     #[test]
//     fn test_eval_numeric() {
//         let mut env = Environment::new();
//         let mut interpreter = Interpreter::new(&mut env);
//         let ast = ASTNode::numeric(7);
//         let result = eval(&mut interpreter, &ast);

//         assert!(result.is_ok());
//         match result {
//             Ok(WhispObj::Int(val)) => assert_eq!(val, 7),
//             _ => panic!("Expected numeric value 7"),
//         }
//     }

//     #[test]
//     fn test_eval_array_index() {
//         let mut env = Environment::new();
//         let mut interpreter = Interpreter::new(&mut env);
//         let ast = ASTNode::array_index(
//             ASTNode::array(vec![
//                 ASTNode::numeric(1), 
//                 ASTNode::numeric(2), 
//                 ASTNode::numeric(3)
//             ]),
//             ASTNode::numeric(1)
//         );
//         let result = eval(&mut interpreter, &ast);

//         assert!(result.is_ok());
//         assert_eq!(result.unwrap(), WhispObj::Int(2));
//     }

//     #[test]
//     fn test_eval_array_index_out_of_bound() {
//         let mut env = Environment::new();
//         let mut interpreter = Interpreter::new(&mut env);
//         let ast = ASTNode::array_index(
//             ASTNode::array(vec![
//                 ASTNode::numeric(1), 
//                 ASTNode::numeric(2), 
//                 ASTNode::numeric(3)
//             ]),
//             ASTNode::numeric(4)
//         );
//         let result = eval(&mut interpreter, &ast);

//         assert!(result.is_err());

//         let err = result.unwrap_err();
//         assert!(err.contains("Index 4 out of bound."));
//     }

//     #[test]
//     fn test_let_binding() {
//         let mut env = Environment::new();
//         let mut interpreter = Interpreter::new(&mut env);
//         let ast = ASTNode::let_binding(
//             ASTNode::identifier("x".to_string()),
//             ASTNode::numeric(42)
//         );
//         let result = eval(&mut interpreter, &ast);

//         assert!(result.is_ok());
//         assert_eq!(env.get("x"), Some(WhispObj::Int(42)));
//     }

//     #[test]
//     fn test_assign_undeclared_variable() {
//         let mut env = Environment::new();
//         let mut interpreter = Interpreter::new(&mut env);
//         let ast = ASTNode::assign(
//             ASTNode::identifier("w".to_string()),
//             ASTNode::numeric(42)
//         );
//         let result = eval(&mut interpreter, &ast);

//         assert!(result.is_err());

//         let err = result.unwrap_err();
//         assert!(err.contains("undeclared variable 'w' referenced."));
//     }

//     #[test]
//     fn test_statements() {
//         let mut env = Environment::new();
//         let mut interpreter = Interpreter::new(&mut env);
//         let ast = ASTNode::statements(vec![
//             ASTNode::let_binding(
//                 ASTNode::identifier("a".to_string()),
//                 ASTNode::numeric(10)
//             ),
//             ASTNode::let_binding(
//                 ASTNode::identifier("b".to_string()),
//                 ASTNode::numeric(20)
//             ),
//             ASTNode::binary_op(
//                 Operation::Add,
//                 ASTNode::identifier("a".to_string()),
//                 ASTNode::identifier("b".to_string())
//             ),
//         ]);
//         let result = eval(&mut interpreter, &ast);

//         assert!(result.is_ok());
//         assert_eq!(result.unwrap(), WhispObj::Int(30));
//     }

//     #[test]
//     fn test_while_loop() {
//         let mut env = Environment::new();
//         let mut interpreter = Interpreter::new(&mut env);
//         let ast = ASTNode::statements(vec![
//             ASTNode::let_binding(
//                 ASTNode::identifier("x".to_string()),
//                 ASTNode::numeric(0)
//             ),
//             ASTNode::while_loop(
//                 ASTNode::binary_op(
//                     Operation::Lt,
//                     ASTNode::identifier("x".to_string()),
//                     ASTNode::numeric(3)
//                 ),
//                 ASTNode::assign(
//                     ASTNode::identifier("x".to_string()),
//                     ASTNode::binary_op(
//                         Operation::Add,
//                         ASTNode::identifier("x".to_string()),
//                         ASTNode::numeric(1)
//                     )
//                 )
//             ),
//         ]);
    
//         let result = eval(&mut interpreter, &ast);
//         assert!(result.is_ok());
//         assert_eq!(env.get("x"), Some(WhispObj::Int(3)));
//     }

//     #[test]
//     fn test_for_loop() {
//         let mut env = Environment::new();
//         let mut interpreter = Interpreter::new(&mut env);
//         let ast = ASTNode::statements(vec![
//             ASTNode::let_binding(
//                 ASTNode::identifier("sum".to_string()),
//                 ASTNode::numeric(0)
//             ),
//             ASTNode::for_loop(
//                 ASTNode::identifier("num".to_string()),
//                 ASTNode::array(vec![
//                     ASTNode::numeric(1),
//                     ASTNode::numeric(2),
//                     ASTNode::numeric(3),
//                 ]),
//                 ASTNode::assign(
//                     ASTNode::identifier("sum".to_string()),
//                     ASTNode::binary_op(
//                         Operation::Add,
//                         ASTNode::identifier("sum".to_string()),
//                         ASTNode::identifier("num".to_string())
//                     )
//                 )
//             ),
//         ]);

//         let result = eval(&mut interpreter, &ast);

//         assert!(result.is_ok());
//         assert_eq!(env.get("sum"), Some(WhispObj::Int(6)));
//     }

//     #[test]
//     fn test_if_statement() {
//         let mut env = Environment::new();
//         let mut interpreter = Interpreter::new(&mut env);
//         let ast = ASTNode::statements(vec![
//             ASTNode::let_binding(
//                 ASTNode::identifier("x".to_string()),
//                 ASTNode::numeric(0)
//             ),
//             ASTNode::if_statement(
//                 ASTNode::boolean(true),
//                 ASTNode::statements(vec![
//                     ASTNode::assign(
//                         ASTNode::identifier("x".to_string()),
//                         ASTNode::numeric(7)
//                     )
//                 ]),
//                 Some(ASTNode::statements(vec![
//                     ASTNode::assign(
//                         ASTNode::identifier("x".to_string()),
//                         ASTNode::numeric(3)
//                     )
//                 ]))
//             )
//         ]);

//         let result = eval(&mut interpreter, &ast);
    
//         assert!(result.is_ok());
//         assert_eq!(env.get("x"), Some(WhispObj::Int(7)));
//     }

//     #[test]
//     fn test_function_call() {
//         let mut env = Environment::new();
//         let mut interpreter = Interpreter::new(&mut env);
//         let ast = ASTNode::statements(vec![
//             ASTNode::function_def(
//                 ASTNode::identifier("return_value"),
//                 vec![
//                     ASTNode::identifier("value")
//                 ],
//                 ASTNode::return_stmt(
//                     ASTNode::identifier("value")
//                 )
//             ),
//             ASTNode::call(
//                 ASTNode::identifier("return_value"),
//                 vec![
//                     ASTNode::string("Hello world!")
//                 ],
//             )
//         ]);

//         let result = eval(&mut interpreter, &ast);

//         assert!(result.is_ok());
//         assert_eq!(result.unwrap(), WhispObj::Str("Hello world!".to_string()));
//     }

//     #[test]
//     fn test_variable_scoping() {
//         let mut env = Environment::new();
//         let mut interpreter = Interpreter::new(&mut env);
//         let ast = ASTNode::statements(vec![
//             ASTNode::let_binding(
//                 ASTNode::identifier("x".to_string()),
//                 ASTNode::numeric(5)
//             ),
//             ASTNode::let_binding(
//                 ASTNode::identifier("y".to_string()),
//                 ASTNode::numeric(3)
//             ),
//             ASTNode::statements(vec![
//                 ASTNode::let_binding(
//                     ASTNode::identifier("x".to_string()),
//                     ASTNode::numeric(7)
//                 ),
//                 ASTNode::assign(
//                     ASTNode::identifier("y".to_string()),
//                     ASTNode::binary_op(
//                         Operation::Add,
//                         ASTNode::identifier("y".to_string()),
//                         ASTNode::identifier("x".to_string())
//                     )
//                 )
//             ]),
//             ASTNode::identifier("y".to_string())
//         ]);

//         let result = eval(&mut interpreter, &ast);

//         assert!(result.is_ok());
//         assert_eq!(result.unwrap(), WhispObj::Int(10));
//     }
// }
