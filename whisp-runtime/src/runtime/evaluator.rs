use crate::value::{ Value };

use whisp_parser::tree::ASTNode;
use whisp_parser::ops::Operation;

pub trait Evaluator {
    fn eval_str(&mut self, node: &ASTNode)          -> Result<Value, String>;
    fn eval_numeric(&mut self, node: &ASTNode)      -> Result<Value, String>;
    fn eval_boolean(&mut self, node: &ASTNode)      -> Result<Value, String>;
    fn eval_array(&mut self, node: &ASTNode)        -> Result<Value, String>;
    fn eval_array_index(&mut self, node: &ASTNode)  -> Result<Value, String>;
    fn eval_identifier(&mut self, node: &ASTNode)   -> Result<Value, String>;
    fn eval_letbinding(&mut self, node: &ASTNode)   -> Result<Value, String>;
    fn eval_assgin(&mut self, node: &ASTNode)       -> Result<Value, String>;
    fn eval_statements(&mut self, node: &ASTNode)   -> Result<Value, String>;
    fn eval_ifstatement(&mut self, node: &ASTNode)  -> Result<Value, String>;
    fn eval_whileloop(&mut self, node: &ASTNode)    -> Result<Value, String>;
    fn eval_forloop(&mut self, node: &ASTNode)      -> Result<Value, String>;
    fn eval_function_def(&mut self, node: &ASTNode) -> Result<Value, String>;
    fn eval_return(&mut self, node: &ASTNode)       -> Result<Value, String>;
    fn eval_function_call(&mut self, node: &ASTNode)-> Result<Value, String>;
    fn eval_binary_op(
        &mut self,
        op: &Operation,
        lhs: &ASTNode,
        rhs: &ASTNode,
    ) -> Result<Value, String>;
}

pub fn eval(
    evaluator: &mut dyn Evaluator,
    node: &ASTNode
) -> Result<Value, String> {
    match node {
        ASTNode::Str { .. }         => evaluator.eval_str(node),
        ASTNode::Bool { .. }        => evaluator.eval_boolean(node),
        ASTNode::Numeric { .. }     => evaluator.eval_numeric(node),
        ASTNode::Array { .. }       => evaluator.eval_array(node),
        ASTNode::ArrayIndex { .. }  => evaluator.eval_array_index(node),
        ASTNode::Identifier { .. }  => evaluator.eval_identifier(node),
        ASTNode::BinaryOp { op, lhs, rhs } => evaluator.eval_binary_op(op, lhs, rhs),
        ASTNode::LetBinding { .. }  => evaluator.eval_letbinding(node),
        ASTNode::Assign { .. }      => evaluator.eval_assgin(node),
        ASTNode::Statements { .. }  => evaluator.eval_statements(node),
        ASTNode::WhileLoop { .. }   => evaluator.eval_whileloop(node),
        ASTNode::ForLoop { .. }     => evaluator.eval_forloop(node),
        ASTNode::IfStatement { .. } => evaluator.eval_ifstatement(node),
        ASTNode::FunctionDef { .. } => evaluator.eval_function_def(node),
        ASTNode::Return { .. }      => evaluator.eval_return(node),
        ASTNode::Call { .. }        => evaluator.eval_function_call(node)
    }
}
