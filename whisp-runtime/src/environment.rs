use std::collections::HashMap;
use crate::value::Value;

pub struct Environment {
    /// Define lexical scope stack to handle nested blocks or function calls. 
    /// Each new scope represents a new block (e.g. function body, loop body).
    ///
    /// Lexical Scoping
    ///
    /// In lexical (static) scoping, a variable’s binding is determined by the
    /// physical structure of the code — that is, by its position in the source
    /// code. When a function is defined, the variables it can access are 
    /// resolved based on its location in the code, regardless of where it's 
    /// called from. Most modern languages (like Rust, Python, JavaScript) use 
    /// lexical scoping.
    ///
    /// Example:
    /// 
    /// ```whisp
    /// let x = 1;
    /// def foo() { let x = 3; println!("{}", x); } // Resolves `x` to 3
    /// ```
    ///
    /// Dynamic Scoping
    ///
    /// In dynamic scoping, a variable’s binding depends on the call stack at 
    /// runtime. When a function is called, it uses variables from the caller’s 
    /// environment, even if those variables are not lexically visible. This 
    /// behavior is mostly found in older or niche languages (like early Lisp 
    /// or Bash).
    ///
    /// Example:
    ///
    /// ```whisp
    /// def foo() { print(x); }
    /// def bar() { let x = 2; foo(); }
    /// bar(); // Under dynamic scoping, `foo` sees x = 2 from `bar`'s scope
    /// ```
    pub stack: Vec<HashMap<String, Value>>,
}

impl Environment {

    pub fn new() -> Self {
        Environment {
            stack: vec![HashMap::new()],
        }
    }

    pub fn enter_scope(&mut self) {
        self.stack.push(HashMap::new());
    }

    pub fn exit_scope(&mut self) {
        self.stack.pop();
    }

    pub fn put(&mut self, name: String, value: Value) {
        if let Some(scope) = self.stack.last_mut() {
            scope.insert(name, value);
        }
    }

    pub fn get(&mut self, name: &str) -> Option<Value> {
        for scope in self.stack.iter().rev() {
            if let Some(val) = scope.get(name) {
                return Some(val.clone());
            }
        }
        None
    }

    pub fn update(&mut self, name: &str, value: Value) -> Result<(), String> {
        for scope in self.stack.iter_mut().rev() {
            if scope.contains_key(name) {
                scope.insert(name.to_string(), value);
                return Ok(());
            }
        }
        
        Err(format!("Undeclared variable '{}' referenced.", name))
    }
}
