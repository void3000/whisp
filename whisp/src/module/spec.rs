use crate::module::loader::ModuleLoader;
use crate::module::object::ModuleObject;

use std::path::PathBuf;

/// A `ModuleSpec` describes the metadata and loader used to resolve and execute a module.
///
/// # Fields
///
/// - `name`: The module’s name.
/// - `loader`: A boxed `ModuleLoader` trait object, enabling dynamic dispatch for custom
///    loading strategies (e.g., file-based, in-memory, remote). This design allows:
///   - Heterogeneous loader implementations in collections (like `Vec<ModuleSpec>`).
///   - Cleaner architecture without pervasive generics.
///   - Runtime flexibility and extensibility, important for supporting Whisp’s multiple
///     import strategies.
/// - `module_search_locations`: Optional locations where the loader can search for the module.
///    Typically used by loaders like file-based importers.
///
/// # Design Note
///
/// While `Box<dyn ModuleLoader>` introduces some runtime cost due to dynamic dispatch, the
/// tradeoff is worthwhile for:
/// - Simpler system design
/// - Better extensibility
/// - Cleaner separation of concerns across module types
pub struct ModuleSpec {
    pub name: String,
    pub loader: Box<dyn ModuleLoader>,
    pub module_search_locations: Vec<PathBuf>,
}

impl ModuleSpec {
    pub fn new(
        name: String, 
        loader: Box<dyn ModuleLoader>, 
        module_search_locations: Vec<PathBuf>
    ) -> Self {
        Self {
            name, 
            loader,
            module_search_locations
        }
    }
}
