use crate::module::loader::ModuleLoader;
use crate::module::object::ModuleObject;
use crate::module::spec::ModuleSpec;
use crate::whisp::Whisp;

use whisp_parser::tree::ASTNode;
use whisp_runtime::value::Value;

use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use std::{
    fs::File,
    io::{
        self,
        Read
    },
    path::PathBuf
};
use std::env;

pub struct BuiltinImporter {
    pub whisp: Whisp
}

impl BuiltinImporter {
    pub fn new() -> Self {
        let whisp = Whisp::new();

        Self { whisp }
    }

    pub fn exec(&mut self, ast: &ASTNode, module: Rc<ModuleObject>) -> Result<(), String> {
        let _ = self.whisp
            .eval(&ast)
            .map_err(|err| format!("ModuleExecError: {}", err))?;

        let mut env = self.whisp.env.borrow_mut();
        let mut scope = module.scope.borrow_mut();

        while let Some(frame) = env.stack.pop() {
            scope.extend(frame);
        }

        Ok(())
    }

    pub fn load_from_locations(
        filename: &str,
        directories: &[PathBuf],
    ) -> io::Result<String> {
        for directory in directories {
            let mut file_path = directory.clone();
            file_path.push(filename);

            match Self::read_from_source(&file_path) {
                Ok(contents) => return Ok(contents),
                Err(e) => println!("Error reading from {:?}: {}", file_path, e)
            }
        }
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            "File not found in any of the suggested locations",
        ))
    }

    pub fn read_from_source(filepath: &PathBuf) -> io::Result<String> {
        let mut file = File::open(filepath)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents)
    }
}

impl ModuleLoader for BuiltinImporter {
    fn create_module(&self, spec: Rc<ModuleSpec>) -> ModuleObject {
        ModuleObject {
            name: spec.name.clone(),
            spec: spec,
            scope: RefCell::new(HashMap::new())
        }
    }

    fn exec_module(&mut self, module: Rc<ModuleObject>) -> Result<(), String> {
        let locations = module
            .spec
            .module_search_locations
            .as_ref();

        let code = Self::load_from_locations("test.w", locations)
            .map_err(|err| format!("FileLoadError: {}", err))?;

        let ast = self.whisp.parse(&code)
            .map_err(|err| format!("ModuleExecError: {}", err))?;

        self.exec(&ast, module)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::module::spec::ModuleSpec;
    use crate::module::loader::ModuleLoader;
    use std::path::Path;

    #[test]
    fn test_exec_module_loads_and_evals_file() {
        let mut importer = BuiltinImporter::new();

        let spec = Rc::new(
            ModuleSpec {
                name: "test".to_string(),
                loader: Box::new(BuiltinImporter::new()),
                module_search_locations: 
                    vec![
                        PathBuf::from("./tests/")
                    ]
                ,
            }
        );
        let module = Rc::new(importer.create_module(spec));
        let result = importer.exec_module(Rc::clone(&module));

        assert!(result.is_ok());

        let scope = module.scope.borrow();

        assert!(scope.contains_key("maximum"));

        match scope.get("maximum") {
            Some(Value::Function { .. }) => { 
                // Ok we are not going do anything. At least 
                // we know that the function value is pesent.
            }
            Some(val) => panic!("Expected `maximum` to be a Function, got {:?}", val),
            None => panic!("Function `maximum` not found in scope"),
        }
    }
}
