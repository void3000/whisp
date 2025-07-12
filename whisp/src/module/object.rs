use crate::module::spec::ModuleSpec;
use whisp_runtime::value::Value;

use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

pub struct ModuleObject {
    pub name: String,
    pub spec: Rc<ModuleSpec>,
    pub scope: RefCell<HashMap<String, Value>>,
}
