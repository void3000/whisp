use crate::environment::Environment;
use crate::object::WhispObj;

use whisp_parser::symbol::{ SymbolTable, SymbolInfo };
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
    fn call(&self, arg: Vec<WhispObj>) -> WhispObj;
}

pub fn register_builtin(
    fun: Rc<dyn BuiltInFunction>, 
    env: &mut Environment, 
    symb: &mut SymbolTable
) {
    env.put(
        fun.name().to_string(), 
        WhispObj::BuiltInFunction {
            callback: fun.clone()
        }
    );

    symb.define(fun.name().to_string(), SymbolInfo);
}

pub fn register_builtins(env: &mut Environment, symb: &mut SymbolTable) {
    let builtins: Vec<Rc<dyn BuiltInFunction>> = vec![
        Rc::new(PrintFn),
        Rc::new(MaxFn),
        Rc::new(MinFn),
    ];

    for builtin in builtins {
        register_builtin(builtin, env, symb);
    }
}

struct PrintFn;
struct MaxFn;
struct MinFn;

impl BuiltInFunction for PrintFn {
    fn name(&self) -> &'static str {
        "print"
    }

    fn call(&self, args: Vec<WhispObj>) -> WhispObj {
        for arg in args {
            println!("{arg}");
        }
        WhispObj::Void(())
    }
}

impl BuiltInFunction for MaxFn {
    fn name(&self) -> &'static str {
        "max"
    }

    fn call(&self, args: Vec<WhispObj>) -> WhispObj {
        if args.len() != 2 {
            panic!("Expected two paramters but got {}.", args.len());
        }

        let lhs = &args[0];
        let rhs = &args[1];

        match (lhs, rhs) {
            (WhispObj::Int(a), WhispObj::Int(b)) => {
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

    fn call(&self, args: Vec<WhispObj>) -> WhispObj {
        if args.len() != 2 {
            panic!("Expected two paramters but got {}.", args.len());
        }

        let lhs = &args[0];
        let rhs = &args[1];

        match (lhs, rhs) {
            (WhispObj::Int(a), WhispObj::Int(b)) => {
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
    use crate::object::WhispObj;
    use crate::environment::Environment;

    #[test]
    fn test_max() {
        let mut env = Environment::new();
        let mut symbols = SymbolTable::new();

        register_builtin(Rc::new(MaxFn), &mut env, &mut symbols);
        
        let builtin_fun = env.get("max");
        let args = vec![WhispObj::Int(3), WhispObj::Int(4)];

        let result = match builtin_fun {
            Some(WhispObj::BuiltInFunction { callback}) =>  callback.call(args),
            _ => WhispObj::Void(())
        };

        assert_eq!(result, WhispObj::Int(4));
    }
}
