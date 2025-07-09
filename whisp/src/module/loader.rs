use std::rc::Rc;

use crate::module::spec::ModuleSpec;
use crate::module::object::ModuleObject;

/// Trait defining the interface for module loaders.
///
/// Using a `ModuleLoader` trait allows the system to support
/// pluggable and custom module loading strategies. Different
/// implementations can be provided for loading modules from
/// various sources (e.g., filesystem, built-ins, network).
///
/// This design enables extensibility and flexibility by
/// decoupling module loading logic from module management,
/// allowing users to implement and swap loaders without
/// changing core system components.
pub trait ModuleLoader {
    /// Creates a module object based on the given module specification.
    fn create_module(&self, spec: Rc<ModuleSpec>) -> ModuleObject;

    /// Executes the provided module by performing evaluation as needed. 
    /// Returns `Ok(())` on success or an error message.
    fn exec_module(&self, module: Rc<ModuleObject>) -> Result<(), String>;
}
