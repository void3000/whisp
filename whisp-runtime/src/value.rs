use std::fmt;
use whisp_parser::tree::ASTNode;

pub type FunctionArgs = Vec<Value>;
pub type List = Vec<Value>;

pub enum Value {
    Bool(bool),
    Int(i32),
    Str(String),
    Array(List),
    Void(()),
    Function {
        params: FunctionArgs,
        body: Box<ASTNode>,
    },
    Return(Box<Value>),
}

macro_rules! int_bin_op {
    ($self:ident, $other:ident, $op:tt) => {
        match ($self, $other) {
            (Value::Int(a), Value::Int(b)) => Value::Int(a $op b),
            _ => panic!("Expected two integers"),
        }
    };
}

macro_rules! bool_bin_op {
    ($self:ident, $other:ident, $op:tt) => {
        match ($self, $other) {
            (Value::Bool(a), Value::Bool(b)) => Value::Bool(a $op b),
            _ => panic!("Expected two booleans"),
        }
    };
}

macro_rules! compr_bin_op {
    ($self:ident, $other:ident, $op:tt) => {
        match ($self, $other) {
            (Value::Int(a), Value::Int(b)) => Value::Bool(a $op b),
            _ => panic!("Expected two integers for comparison"),
        }
    };
}

impl Value {
    pub fn add(self, other: Value) -> Value { int_bin_op!(self, other, +) }
    pub fn sub(self, other: Value) -> Value { int_bin_op!(self, other, -) }
    pub fn mul(self, other: Value) -> Value { int_bin_op!(self, other, *) }
    pub fn div(self, other: Value) -> Value { int_bin_op!(self, other, /) }

    pub fn and(self, other: Value) -> Value { bool_bin_op!(self, other, &&) }
    pub fn or(self, other: Value) -> Value { bool_bin_op!(self, other, ||) }

    pub fn lt(self, other: Value) -> Value { compr_bin_op!(self, other, <) }
    pub fn gt(self, other: Value) -> Value { compr_bin_op!(self, other, >) }
    pub fn le(self, other: Value) -> Value { compr_bin_op!(self, other, <=) }
    pub fn ge(self, other: Value) -> Value { compr_bin_op!(self, other, >=) }

    pub fn eq(self, other: Value) -> Value {
        Value::Bool(match (self, other) {
            (Value::Int(a), Value::Int(b)) => a == b,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::Str(a), Value::Str(b)) => a == b,
            _ => panic!("Unsupported equality comparison"),
        })
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Bool(b) => write!(f, "{b}"),
            Value::Int(n) => write!(f, "{n}"),
            Value::Str(s) => write!(f, "{s}"),
            Value::Array(arr) => write!(
                f,
                "[{}]",
                arr.iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Value::Void(()) => Ok(()),
            Value::Return(inner) => write!(f, "{inner}"),
            Value::Function { .. } => write!(f, "Function"),
        }
    }
}

impl Clone for Value {
    fn clone(&self) -> Self {
        match self {
            Value::Bool(b) => Value::Bool(*b),
            Value::Int(n) => Value::Int(*n),
            Value::Str(s) => Value::Str(s.clone()),
            Value::Array(arr) => Value::Array(arr.clone()),
            Value::Void(()) => Value::Void(()),
            Value::Return(inner) => Value::Return(Box::new((**inner).clone())),
            Value::Function { params, body } => Value::Function {
                params: params.clone(),
                body: body.clone(),
            },
        }
    }
}
