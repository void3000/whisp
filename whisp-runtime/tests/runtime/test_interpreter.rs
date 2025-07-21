use whisp_runtime::runtime::interpreter::Interpreter;
use whisp_runtime::runtime::evaluator::eval;
use whisp_runtime::environment::Environment;
use whisp_runtime::object::WhispObj;
use whisp_parser::tree::ASTNode;
use whisp_parser::ops::Operation;

#[test]
fn test_interpreter_numeric() {
    let mut env = Environment::new();
    let interpreter = Interpreter::new(&mut env);
    let ast = ASTNode::Numeric { value: 6 };

    let result = eval(&interpreter, &ast);
    
    match result {
        Ok(WhispObj::Int(val)) => assert_eq!(val, 6),
        _ => panic!("Expected an integer value"),
    }
}

#[test]
fn test_interpreter_string() {
    let mut env = Environment::new();
    let interpreter = Interpreter::new(&mut env);
    let ast = ASTNode::Str { value: "hello".into() };

    let result = eval(&interpreter, &ast);

    match result {
        Ok(WhispObj::Str(val)) => assert_eq!(val, "hello"),
        _ => panic!("Expected a string value"),
    }
}

#[test]
fn test_interpreter_boolean() {
    let mut env = Environment::new();
    let interpreter = Interpreter::new(&mut env);
    let ast = ASTNode::Bool { value: true };

    let result = eval(&interpreter, &ast);

    match result {
        Ok(WhispObj::Bool(val)) => assert_eq!(val, true),
        _ => panic!("Expected a booelan value"),
    }
}

#[test]
fn test_interpreter_binary_op_addition() {
    let mut env = Environment::new();
    let interpreter = Interpreter::new(&mut env);
    let ast = ASTNode::BinaryOp {
        op: Operation::Add,
        lhs: Box::new(ASTNode::Numeric { value: 3 }),
        rhs: Box::new(ASTNode::Numeric { value: 4 }),
    };

    let result = eval(&interpreter, &ast);

    match result {
        Ok(WhispObj::Int(val)) => assert_eq!(val, 7),
        _ => panic!("Expected an integer value from addition"),
    }
}
