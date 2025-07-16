use crate::ops::Operation;

#[derive(Clone, Debug, PartialEq)]
pub enum ASTNode {
    Bool { value: bool },
    Numeric { value: i32 },
    Str { value: String },
    Identifier { name: String },
    Statements { 
        stmts: Vec<ASTNode> 
    },
    Array { 
        elements: Vec<ASTNode> 
    },
    ArrayIndex {
        arr: Box<ASTNode>, 
        index: Box<ASTNode> 
    },
    BinaryOp {
        op: Operation,
        lhs: Box<ASTNode>,
        rhs: Box<ASTNode>
    },
    Import {
        path: Vec<ASTNode>
    },
    Assign { 
        identifier: Box<ASTNode>, 
        body: Box<ASTNode> 
    },
    LetBinding { 
        identifier: Box<ASTNode>,
        body: Box<ASTNode>
    },
    WhileLoop { 
        cond: Box<ASTNode>,
        body: Box<ASTNode>
    },
    ForLoop {
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

impl ASTNode {
     pub fn statements(stmts: Vec<ASTNode>) -> Self {
        ASTNode::Statements { stmts }
    }

    pub fn numeric(value: i32) -> Self {
        ASTNode::Numeric { value }
    }

    pub fn boolean(value: bool) -> Self {
        ASTNode::Bool { value }
    }

    pub fn string(value: impl Into<String>) -> Self {
        ASTNode::Str { value: value.into() }
    }

    pub fn array(elements: Vec<ASTNode>) -> Self {
        ASTNode::Array { elements }
    }

    pub fn identifier(name: impl Into<String>) -> Self {
        ASTNode::Identifier { name: name.into() }
    }

    pub fn binary_op(
        op: Operation, 
        lhs: ASTNode, 
        rhs: ASTNode
    ) -> Self {
        ASTNode::BinaryOp {
            op,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        }
    }

    pub fn assign(identifier: ASTNode, body: ASTNode) -> Self {
        ASTNode::Assign {
            identifier: Box::new(identifier),
            body: Box::new(body),
        }
    }

    pub fn import(path: Vec<ASTNode>) -> Self {
        ASTNode::Import { path: path }
    }

    pub fn let_binding(identifier: ASTNode, body: ASTNode) -> Self {
        ASTNode::LetBinding {
            identifier: Box::new(identifier),
            body: Box::new(body),
        }
    }

    pub fn array_index(arr: ASTNode, index: ASTNode) -> Self {
        ASTNode::ArrayIndex {
            arr: Box::new(arr),
            index: Box::new(index),
        }
    }

    pub fn while_loop(cond: ASTNode, body: ASTNode) -> Self {
        ASTNode::WhileLoop {
            cond: Box::new(cond),
            body: Box::new(body),
        }
    }

    pub fn for_loop(
        var: ASTNode,
        itr: ASTNode,
        body: ASTNode
    ) -> Self {
        ASTNode::ForLoop {
            var: Box::new(var),
            itr: Box::new(itr),
            body: Box::new(body),
        }
    }

    pub fn if_statement(
        cond: ASTNode, 
        then_branch: ASTNode, 
        else_branch: Option<ASTNode>
    ) -> Self {
        ASTNode::IfStatement {
            cond: Box::new(cond),
            then_branch: Box::new(then_branch),
            else_branch: else_branch.map(Box::new),
        }
    }

    pub fn function_def(
        name: ASTNode, 
        params: Vec<ASTNode>, 
        body: ASTNode
    ) -> Self {
        ASTNode::FunctionDef {
            name: Box::new(name),
            params,
            body: Box::new(body),
        }
    }

    pub fn return_stmt(value: ASTNode) -> Self {
        ASTNode::Return {
            value: Box::new(value),
        }
    }

    pub fn call(name: ASTNode, args: Vec<ASTNode>) -> Self {
        ASTNode::Call {
            name: Box::new(name),
            args,
        }
    }
}
