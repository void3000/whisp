use std::rc::Rc;
use std::fmt;
use whisp_parser::tree::ASTNode;
use crate::runtime::builtin::BuiltInFunction;

pub enum WhispObj {
    Bool(bool),
    Int(i32),
    Str(String),
    Array(Vec<WhispObj>),
    Void(()),
    Function {
        params: Vec<ASTNode>,
        body: Box<ASTNode>,
    },
    BuiltInFunction {
        callback: Rc<dyn BuiltInFunction>
    },
    Return(Box<WhispObj>),
}

macro_rules! int_bin_op {
    ($self:ident, $other:ident, $op:tt) => {
        match ($self, $other) {
            (WhispObj::Int(a), WhispObj::Int(b)) => WhispObj::Int(a $op b),
            _ => panic!("Expected two integers"),
        }
    };
}

macro_rules! bool_bin_op {
    ($self:ident, $other:ident, $op:tt) => {
        match ($self, $other) {
            (WhispObj::Bool(a), WhispObj::Bool(b)) => WhispObj::Bool(a $op b),
            _ => panic!("Expected two booleans"),
        }
    };
}

macro_rules! compr_bin_op {
    ($self:ident, $other:ident, $op:tt) => {
        match ($self, $other) {
            (WhispObj::Int(a), WhispObj::Int(b)) => WhispObj::Bool(a $op b),
            _ => panic!("Expected two integers for comparison"),
        }
    };
}

impl WhispObj {
    pub fn add(self, other: WhispObj) -> WhispObj { int_bin_op!(self, other, +) }
    pub fn sub(self, other: WhispObj) -> WhispObj { int_bin_op!(self, other, -) }
    pub fn mul(self, other: WhispObj) -> WhispObj { int_bin_op!(self, other, *) }
    pub fn div(self, other: WhispObj) -> WhispObj { int_bin_op!(self, other, /) }
    pub fn modulo(self, other: WhispObj) -> WhispObj { int_bin_op!(self, other, %) }

    pub fn and(self, other: WhispObj) -> WhispObj { bool_bin_op!(self, other, &&) }
    pub fn or(self, other: WhispObj) -> WhispObj { bool_bin_op!(self, other, ||) }

    pub fn lt(self, other: WhispObj) -> WhispObj { compr_bin_op!(self, other, <) }
    pub fn gt(self, other: WhispObj) -> WhispObj { compr_bin_op!(self, other, >) }
    pub fn le(self, other: WhispObj) -> WhispObj { compr_bin_op!(self, other, <=) }
    pub fn ge(self, other: WhispObj) -> WhispObj { compr_bin_op!(self, other, >=) }

    pub fn eq(self, other: WhispObj) -> WhispObj {
        WhispObj::Bool(match (self, other) {
            (WhispObj::Int(a), WhispObj::Int(b)) => a == b,
            (WhispObj::Bool(a), WhispObj::Bool(b)) => a == b,
            (WhispObj::Str(a), WhispObj::Str(b)) => a == b,
            _ => panic!("Unsupported equality comparison"),
        })
    }
}

impl fmt::Display for WhispObj {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WhispObj::Bool(b) => write!(f, "{b}"),
            WhispObj::Int(n) => write!(f, "{n}"),
            WhispObj::Str(s) => write!(f, "{s}"),
            WhispObj::Array(arr) => write!(
                f,
                "[{}]",
                arr.iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            WhispObj::Void(()) => Ok(()),
            WhispObj::Return(inner) => write!(f, "{inner}"),
            WhispObj::Function { .. } => write!(f, "Function"),
            WhispObj::BuiltInFunction { .. } => write!(f, "Builtin function"),
        }
    }
}

impl Clone for WhispObj {
    fn clone(&self) -> Self {
        match self {
            WhispObj::Bool(b) => WhispObj::Bool(*b),
            WhispObj::Int(n) => WhispObj::Int(*n),
            WhispObj::Str(s) => WhispObj::Str(s.clone()),
            WhispObj::Array(arr) => WhispObj::Array(arr.clone()),
            WhispObj::Void(()) => WhispObj::Void(()),
            WhispObj::Return(inner) => WhispObj::Return(Box::new((**inner).clone())),
            WhispObj::Function { params, body } => WhispObj::Function {
                params: params.clone(),
                body: body.clone(),
            },
            WhispObj::BuiltInFunction { callback } => WhispObj::BuiltInFunction {
                callback: callback.clone(),
            },
        }
    }
}

impl fmt::Debug for WhispObj {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WhispObj::BuiltInFunction { .. } => write!(f, "BuiltInFunction(<opaque>)"),
            other => write!(f, "{:?}", other),
        }
    }
}

impl PartialEq for WhispObj {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (WhispObj::Bool(a), WhispObj::Bool(b)) => a == b,
            (WhispObj::Int(a), WhispObj::Int(b)) => a == b,
            (WhispObj::Str(a), WhispObj::Str(b)) => a == b,
            (WhispObj::Array(a), WhispObj::Array(b)) => a == b,
            (WhispObj::Void(_), WhispObj::Void(_)) => true,
            (WhispObj::Return(a), WhispObj::Return(b)) => a == b,
            (WhispObj::BuiltInFunction { callback: a }, 
             WhispObj::BuiltInFunction { callback: b }) => {
                Rc::ptr_eq(a, b)
            }
            (WhispObj::Function { .. }, WhispObj::Function { .. }) => false,

            _ => false,
        }
    }
}
