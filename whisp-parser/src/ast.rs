use crate::ops::Operation;

#[derive(Clone, Debug)]
pub enum ASTNode {
    Sequence { 
        stmts: Vec<ASTNode> 
    },
    Bool { value: bool },
    Numeric { value: i32 },
    Str { value: String },
    Array { 
        elements: Vec<ASTNode> 
    },
    ArrayIndex {
        arr: Box<ASTNode>, 
        index: Box<ASTNode> 
    },
    Identifier { name: String },
    BinaryOp {
        op: Operation,
        lhs: Box<ASTNode>,
        rhs: Box<ASTNode>
    },
    Assign { 
        identifier: Box<ASTNode>, 
        body: Box<ASTNode> 
    },
    LetBinding { 
        identifier: Box<ASTNode>,
        body: Box<ASTNode>
    },
    WhileLoopStatement { 
        cond: Box<ASTNode>,
        body: Box<ASTNode>
    },
    ForLoopStatement {
        itr: Box<ASTNode>,
        var: Box<ASTNode>,
        body: Box<ASTNode>
    },
    IfStatement { 
        cond: Box<ASTNode>,
        then_branch: Box<ASTNode>,
        else_branch: Option<Box<ASTNode>> 
    },
    FunctionDef {
        name: Box<ASTNode>,
        params: Vec<ASTNode>,
        body: Box<ASTNode>,
    },
    Return {
        value: Box<ASTNode>
    },
    Call {
        name: Box<ASTNode>,
        args: Vec<ASTNode>
    }
}
