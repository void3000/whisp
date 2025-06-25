use whisp_runtime::value_type::Value;

#[test]
fn test_addition() {
    let a = Value::Int(5);
    let b = Value::Int(3);
    let result = a.add(b);

    match result {
        Value::Int(n) => assert_eq!(n, 8),
        _ => panic!("Expected Value::Int"),
    }
}

#[test]
fn test_subtraction() {
    let a = Value::Int(5);
    let b = Value::Int(3);
    let result = a.sub(b);

    match result {
        Value::Int(n) => assert_eq!(n, 2),
        _ => panic!("Expected Value::Int"),
    }
}

#[test]
fn test_multiplication() {
    let a = Value::Int(5);
    let b = Value::Int(3);
    let result = a.mul(b);

    match result {
        Value::Int(n) => assert_eq!(n, 15),
        _ => panic!("Expected Value::Int"),
    }
}

#[test]
fn test_division() {
    let a = Value::Int(6);
    let b = Value::Int(3);
    let result = a.div(b);

    match result {
        Value::Int(n) => assert_eq!(n, 2),
        _ => panic!("Expected Value::Int"),
    }
}

#[test]
fn test_equality() {
    let a = Value::Int(5);
    let b = Value::Int(5);
    let result = a.eq(b);

    match result {
        Value::Bool(true) => (),
        _ => panic!("Expected Value::Bool(true)"),
    }
}

#[test]
fn test_less_than() {
    let a = Value::Int(3);
    let b = Value::Int(5);
    let result = a.lt(b);

    match result {
        Value::Bool(true) => (),
        _ => panic!("Expected Value::Bool(true)"),
    }
}

#[test]
fn test_greater_than() {
    let a = Value::Int(5);
    let b = Value::Int(3);
    let result = a.gt(b);

    match result {
        Value::Bool(true) => (),
        _ => panic!("Expected Value::Bool(true)"),
    }
}

#[test]
fn test_less_than_or_equal() {
    let a = Value::Int(3);
    let b = Value::Int(5);
    let result = a.le(b);

    match result {
        Value::Bool(true) => (),
        _ => panic!("Expected Value::Bool(true)"),
    }
}

#[test]
fn test_greater_than_or_equal() {
    let a = Value::Int(5);
    let b = Value::Int(3);
    let result = a.ge(b);

    match result {
        Value::Bool(true) => (),
        _ => panic!("Expected Value::Bool(true)"),
    }
}

#[test]
fn test_and() {
    let a = Value::Bool(true);
    let b = Value::Bool(false);
    let result = a.and(b);

    match result {
        Value::Bool(false) => (),
        _ => panic!("Expected Value::Bool(false)"),
    }
}

#[test]
fn test_or() {
    let a = Value::Bool(true);
    let b = Value::Bool(false);
    let result = a.or(b);

    match result {
        Value::Bool(true) => (),
        _ => panic!("Expected Value::Bool(true)"),
    }
}
