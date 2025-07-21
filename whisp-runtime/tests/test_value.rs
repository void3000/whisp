use whisp_runtime::object::WhispObj;

#[test]
fn test_addition() {
    let a = WhispObj::Int(5);
    let b = WhispObj::Int(3);
    let result = a.add(b);

    match result {
        WhispObj::Int(n) => assert_eq!(n, 8),
        _ => panic!("Expected WhispObj::Int"),
    }
}

#[test]
fn test_subtraction() {
    let a = WhispObj::Int(5);
    let b = WhispObj::Int(3);
    let result = a.sub(b);

    match result {
        WhispObj::Int(n) => assert_eq!(n, 2),
        _ => panic!("Expected WhispObj::Int"),
    }
}

#[test]
fn test_multiplication() {
    let a = WhispObj::Int(5);
    let b = WhispObj::Int(3);
    let result = a.mul(b);

    match result {
        WhispObj::Int(n) => assert_eq!(n, 15),
        _ => panic!("Expected WhispObj::Int"),
    }
}

#[test]
fn test_division() {
    let a = WhispObj::Int(6);
    let b = WhispObj::Int(3);
    let result = a.div(b);

    match result {
        WhispObj::Int(n) => assert_eq!(n, 2),
        _ => panic!("Expected WhispObj::Int"),
    }
}

#[test]
fn test_equality() {
    let a = WhispObj::Int(5);
    let b = WhispObj::Int(5);
    let result = a.eq(b);

    match result {
        WhispObj::Bool(true) => (),
        _ => panic!("Expected WhispObj::Bool(true)"),
    }
}

#[test]
fn test_less_than() {
    let a = WhispObj::Int(3);
    let b = WhispObj::Int(5);
    let result = a.lt(b);

    match result {
        WhispObj::Bool(true) => (),
        _ => panic!("Expected WhispObj::Bool(true)"),
    }
}

#[test]
fn test_greater_than() {
    let a = WhispObj::Int(5);
    let b = WhispObj::Int(3);
    let result = a.gt(b);

    match result {
        WhispObj::Bool(true) => (),
        _ => panic!("Expected WhispObj::Bool(true)"),
    }
}

#[test]
fn test_less_than_or_equal() {
    let a = WhispObj::Int(3);
    let b = WhispObj::Int(5);
    let result = a.le(b);

    match result {
        WhispObj::Bool(true) => (),
        _ => panic!("Expected WhispObj::Bool(true)"),
    }
}

#[test]
fn test_greater_than_or_equal() {
    let a = WhispObj::Int(5);
    let b = WhispObj::Int(3);
    let result = a.ge(b);

    match result {
        WhispObj::Bool(true) => (),
        _ => panic!("Expected WhispObj::Bool(true)"),
    }
}

#[test]
fn test_and() {
    let a = WhispObj::Bool(true);
    let b = WhispObj::Bool(false);
    let result = a.and(b);

    match result {
        WhispObj::Bool(false) => (),
        _ => panic!("Expected WhispObj::Bool(false)"),
    }
}

#[test]
fn test_or() {
    let a = WhispObj::Bool(true);
    let b = WhispObj::Bool(false);
    let result = a.or(b);

    match result {
        WhispObj::Bool(true) => (),
        _ => panic!("Expected WhispObj::Bool(true)"),
    }
}
