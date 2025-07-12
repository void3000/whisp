use std::collections::HashMap;
use crate::value::Value;

#[derive(Clone)]
pub struct Environment {
    /// Represents the environment for variable bindings in Whisp.
    ///
    /// This structure implements **lexical scoping** via a stack of scopes,
    /// where each scope is represented by a `HashMap<String, Value>`. New
    /// scopes are pushed onto the stack during function calls or blocks, and
    /// popped afterward.
    ///
    /// ---
    ///
    /// ### Lexical Scoping
    /// In lexical (or static) scoping, a variable’s binding is determined by the
    /// physical structure of the code — that is, by its position in the source.
    /// When a function is defined, the variables it can access are resolved based
    /// on where the function is written, not where it is called.
    ///
    /// Most modern languages (like Rust, Python, JavaScript) use lexical scoping.
    ///
    /// Example:
    /// ```whisp
    /// let x = 1;
    /// def foo() { let x = 3; println(x); }
    /// foo(); // Resolves `x` to 3 inside `foo`
    /// ```
    ///
    /// ---
    ///
    /// ### Dynamic Scoping (Not the default in Whisp)
    /// In dynamic scoping, a variable’s binding depends on the runtime call stack.
    /// A function can see variables from the caller’s environment, even if they are
    /// not lexically visible. This is typical of older languages (like early Lisp or Bash).
    ///
    /// Example:
    /// ```whisp
    /// def foo() { print(x); }
    /// def bar() { let x = 2; foo(); }
    /// bar(); // Under dynamic scoping, `foo` sees x = 2 from `bar`
    /// ```
    ///
    /// Although Whisp supports dynamic scoping for certain constructs, its primary
    /// model is lexical.
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
        
        Err(format!("undeclared variable '{}' referenced.", name))
    }
}
