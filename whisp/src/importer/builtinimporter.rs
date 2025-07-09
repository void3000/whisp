use crate::module::loader::ModuleLoader;
use crate::module::object::ModuleObject;
use crate::module::spec::ModuleSpec;

use std::collections::HashMap;
use std::rc::Rc;
use std::fs;
use std::env;

pub struct BuiltinImporter;

impl ModuleLoader for BuiltinImporter {
    fn create_module(&self, spec: Rc<ModuleSpec>) -> ModuleObject {
        ModuleObject {
            name: spec.name.clone(),
            spec: spec,
            scope: HashMap::new()
        }
    }

    fn exec_module(&self, module: Rc<ModuleObject>) -> Result<(), String> {        
        Ok(())
    }
}
