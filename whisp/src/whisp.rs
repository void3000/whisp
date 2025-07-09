use std::cell::RefCell;
use std::rc::Rc;

use whisp_lexer::lexer::{Lexer, Collection};
use whisp_parser::{
    parser::ll_parser::{LLParser, Parser},
    symbol::SymbolTable,
    tree::ASTNode,
};
use whisp_runtime::{
    environment::Environment,
    runtime::{
        builtin, 
        evaluator, 
        evaluator::Evaluator, 
        interpreter::Interpreter
    },
    value::Value,
};


type ParseResponse = ASTNode;
type EvalResponse  = Value;
type ResponseErr   = String;

pub struct Whisp {
    pub eval: Box<dyn Evaluator>,
    pub env: Rc<RefCell<Environment>>,
    pub sym: Rc<RefCell<SymbolTable>>
}

impl<'a> Whisp {
    pub fn new() -> Self {
        /// Global tables
        let sym = Rc::new(RefCell::new(SymbolTable::new()));
        let env = Rc::new(RefCell::new(Environment::new()));

        /// Register builtin functions
        builtin::register_builtins(&mut env.borrow_mut(), &mut sym.borrow_mut());

        /// Ready the interpreter
        let eval = Box::new(Interpreter::new(Rc::clone(&env)));

        Self { 
            eval, 
            env, 
            sym 
        }
    }

    /// Parses the given source code string into an abstract syntax tree (AST).
    /// 
    /// # Arguments
    /// * `source_code` - A string slice containing the source code to parse.
    /// 
    /// # Returns
    /// * `Result<ParseResponse, ResponseErr>` - On success, returns the root AST node;
    ///   on failure, returns a parsing error as a string.
    pub fn parse(&self, source_code: &'a str) -> Result<ParseResponse, ResponseErr> {
        let lexer = Lexer::new(source_code);

        let stream = lexer.stream();

        /// Rust complains about short lived temporary variable.
        let mut binding = self.sym.borrow_mut();
        let mut parser = LLParser::new(stream, &mut binding);

        parser.parse()
    }

    /// Evaluates the given abstract syntax tree (AST) node and returns the resulting value.
    ///
    /// # Arguments
    /// * `ast` - A reference to the AST node to evaluate.
    ///
    /// # Returns
    /// * `Result<EvalResponse, ResponseErr>` - On success, returns the evaluated value;
    ///   on failure, returns an evaluation error as a string.
    pub fn eval(&mut self, ast: &ASTNode) -> Result<EvalResponse, ResponseErr> {
        evaluator::eval(self.eval.as_mut(), ast)
    }
}
