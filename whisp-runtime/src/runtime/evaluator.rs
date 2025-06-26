use crate::value::{ Value };
use whisp_parser::tree::ASTNode;
use whisp_parser::ops::Operation;

pub trait Evaluator {
    fn eval_str(&self, node: &ASTNode) -> Result<Value, String>;
    fn eval_numeric(&self, node: &ASTNode) -> Result<Value, String>;
    fn eval_boolean(&self, node: &ASTNode) -> Result<Value, String>;
    fn eval_binary_op(
        &self,
        op: &Operation,
        lhs: &ASTNode,
        rhs: &ASTNode,
    ) -> Result<Value, String>;
}

pub fn eval(
    evaluator: &dyn Evaluator,
    node: &ASTNode
) -> Result<Value, String> {
    match node {
        ASTNode::Str { .. }  => evaluator.eval_str(&node),
        ASTNode::Bool { .. } => evaluator.eval_boolean(&node),
        ASTNode::Numeric { .. } => evaluator.eval_numeric(&node),
        ASTNode::BinaryOp { op, lhs, rhs } => evaluator.eval_binary_op(op, lhs, rhs),
        _ => Err("Unsupported ASTNode type".to_string()),
    }
}
