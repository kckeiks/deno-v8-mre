use deno_core::{
    ModuleLoadResponse, ModuleLoader, ModuleSource, ModuleSpecifier, ModuleType,
    RequestedModuleType,
};
use std::fs;

pub struct MockModuleLoader {}

impl MockModuleLoader {
    pub fn new() -> Self {
        Self {}
    }
}

impl ModuleLoader for MockModuleLoader {
    fn resolve(
        &self,
        specifier: &str,
        referrer: &str,
        _kind: deno_core::ResolutionKind,
    ) -> Result<ModuleSpecifier, anyhow::Error> {
        let import = deno_core::resolve_import(specifier, referrer)?;
        Ok(import)
    }

    fn load(
        &self,
        module_specifier: &ModuleSpecifier,
        _maybe_referrer: Option<&ModuleSpecifier>,
        _is_dyn_import: bool,
        _requested_module_type: RequestedModuleType,
    ) -> ModuleLoadResponse {
        let source = fs::read("oom_test.js").unwrap().into_boxed_slice();
        ModuleLoadResponse::Sync(Ok(ModuleSource::new(
            ModuleType::JavaScript,
            deno_core::ModuleSourceCode::Bytes(source.into()),
            &module_specifier,
            None,
        )))
    }
}
