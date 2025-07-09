use crate::module::loader::ModuleLoader;
use crate::module::object::ModuleObject;

pub struct ModuleSpec {
    pub name: String,
    /// Use `Box<dyn ModuleLoader>` in `ModuleSpec` to enable dynamic dispatch 
    /// for different types of module loaders (e.g., file-based, built-in, remote).
    /// This design allows us to store various loader implementations behind a 
    /// common interface without requiring generics throughout the system. It 
    /// also enables us to collect different loaders in a single data structure,
    /// such as a `Vec<ModuleSpec>`, which would be impossible with generics alone.
    /// While using trait objects introduces a small runtime cost due to dynamic
    /// dispatch, it simplifies the architecture, avoids excessive code duplication,
    /// and makes the system more extensible — especially useful in a language like
    /// Whisp that supports multiple import strategies.
    pub loader: Box<dyn ModuleLoader>,
    pub module_search_locations: Option<Vec<String>>,
}

impl ModuleSpec {
    pub fn new(
        name: String, 
        loader: Box<dyn ModuleLoader>, 
        module_search_locations: Option<Vec<String>>
    ) -> Self {
        Self {
            name, 
            loader,
            module_search_locations
        }
    }
}
