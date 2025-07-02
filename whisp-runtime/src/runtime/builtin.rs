use crate::environment::Environment;
use crate::value::Value;

use std::rc::Rc;

/// Lessons:
///
/// (1) Trait objects (`dyn`) can't be cloned directly
/// (2) Use `dyn Trait` for runtime polymorphism:
///     - Accept multiple types implementing a trait
///     - Dispatch calls at runtime
///     - Store different impls in the same container
/// (3) Use `Rc<dyn Trait>` to:
///     - Enable cloning
///     - Avoid cyclic ownership issues
/// (4) Prefer storing trait object directly in enum, not as a fn pointer
/// (5) Rust doesn't support trait objects of generic traits.
pub trait BuiltInFunction {
    fn name(&self) -> &'static str;
    fn call(&self, arg: Vec<Value>) -> Value;
}

fn register_builtin(fun: Rc<dyn BuiltInFunction>, env: &mut Environment) {
    env.put(
        fun.name().to_string(), 
        Value::BuiltInFunction {
            callback: fun
        }
    );
}

fn register_builtins(env: &mut Environment) {
    let builtins: Vec<Rc<dyn BuiltInFunction>> = vec![
        Rc::new(PrintFn),
        Rc::new(MaxFn),
        Rc::new(MinFn),
    ];

    for builtin in builtins {
        register_builtin(builtin, env);
    }
}

struct PrintFn;
struct MaxFn;
struct MinFn;

impl BuiltInFunction for PrintFn {
    fn name(&self) -> &'static str {
        "print"
    }

    fn call(&self, args: Vec<Value>) -> Value {
        for arg in args {
            println!("{arg}");
        }
        Value::Void(())
    }
}

impl BuiltInFunction for MaxFn {
    fn name(&self) -> &'static str {
        "max"
    }

    fn call(&self, args: Vec<Value>) -> Value {
        if args.len() != 2 {
            panic!("Expected two paramters but got {}.", args.len());
        }

        let lhs = &args[0];
        let rhs = &args[1];

        match (lhs, rhs) {
            (Value::Int(a), Value::Int(b)) => {
                if a > b {
                    lhs.clone()
                } else {
                    rhs.clone()
                }
            },
            _ => panic!("Expected numeric types.")
        }
    }
}

impl BuiltInFunction for MinFn {
    fn name(&self) -> &'static str {
        "min"
    }

    fn call(&self, args: Vec<Value>) -> Value {
        if args.len() != 2 {
            panic!("Expected two paramters but got {}.", args.len());
        }

        let lhs = &args[0];
        let rhs = &args[1];

        match (lhs, rhs) {
            (Value::Int(a), Value::Int(b)) => {
                if a < b {
                    lhs.clone()
                } else {
                    rhs.clone()
                }
            },
            _ => panic!("Expected numeric types.")
        }
    }
}


#[cfg(test)]
mod test_builtin_functions {
    use super::*;
    use crate::value::Value;
    use crate::environment::Environment;

    #[test]
    fn test_max() {
        let mut env = Environment::new();

        register_builtin(Rc::new(MaxFn), &mut env);
        
        let builtin_fun = env.get("max");
        let args = vec![Value::Int(3), Value::Int(4)];

        let result = match builtin_fun {
            Some(Value::BuiltInFunction { callback}) =>  callback.call(args),
            _ => Value::Void(())
        };

        assert_eq!(result, Value::Int(4));
    }
}
