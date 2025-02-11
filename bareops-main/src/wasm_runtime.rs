use crate::error::BareopsError;
use bareops_lang::{Identifier, PluginOption, Value};
use log::debug;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use thiserror::Error;
use wasmtime::component::{Component, Linker, ResourceTable, Val};
use wasmtime::{Config, Engine, Store};
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder, WasiView};

#[derive(Error, Debug)]
pub enum WasmRuntimeError {
    #[error("Failed create engine")]
    EngineCreation(String),
    #[error("Failed to run component")]
    ComponentExecution(String),
}

struct WasmState {
    ctx: WasiCtx,
    table: ResourceTable,
}

impl WasiView for WasmState {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.ctx
    }
}

pub struct WasmRuntime<'a> {
    engine: Engine,
    paths: Vec<&'a Path>,
    store: Store<WasmState>,
    linker: Linker<WasmState>,
    components: HashMap<String, Component>,
}

impl From<WasmRuntimeError> for BareopsError {
    fn from(err: WasmRuntimeError) -> Self {
        dbg!(&err);
        match err {
            WasmRuntimeError::EngineCreation(s) => BareopsError::Init(s),
            WasmRuntimeError::ComponentExecution(s) => BareopsError::TaskbookExecution(s),
        }
    }
}

impl<'a> WasmRuntime<'a> {
    pub fn new(config: Config) -> Result<Self, WasmRuntimeError> {
        let engine =
            Engine::new(&config).map_err(|e| WasmRuntimeError::EngineCreation(e.to_string()))?;

        let mut linker = Linker::<WasmState>::new(&engine);
        wasmtime_wasi::add_to_linker_async(&mut linker)
            .map_err(|e| WasmRuntimeError::EngineCreation(e.to_string()))?;

        let mut builder = WasiCtxBuilder::new();
        builder.inherit_stdio();

        let store = Store::new(
            &engine,
            WasmState {
                ctx: builder.build(),
                table: ResourceTable::new(),
            },
        );

        Ok(WasmRuntime {
            engine,
            linker,
            store,
            paths: Vec::new(),
            components: HashMap::new(),
        })
    }

    pub fn set_search_paths(&mut self, paths: &'a [impl AsRef<Path>]) {
        self.paths = paths.iter().map(|p| p.as_ref()).collect();
    }

    pub async fn run_component(
        &mut self,
        name: &str,
        options: &[PluginOption],
    ) -> Result<(), WasmRuntimeError> {
        let path = self.find_file(format!("{}.wasm", name)).ok_or(
            WasmRuntimeError::ComponentExecution(format!("Cannot find component {}", name)),
        )?;

        if !self.components.contains_key(name) {
            debug!("Compiling wasm component {:?}", path);
            let component = Component::from_file(&self.engine, &path).map_err(|e| {
                WasmRuntimeError::ComponentExecution(format!("Cannot read component file: {}", e))
            })?;
            self.components.insert(name.to_string(), component);
        }
        let Some(component) = self.components.get(name) else {
            return Err(WasmRuntimeError::ComponentExecution(format!(
                "Cannot compile component {:?}",
                name
            )));
        };

        let instance = self
            .linker
            .instantiate_async(&mut self.store, component)
            .await
            .map_err(|e| {
                WasmRuntimeError::ComponentExecution(format!("Cannot instantiate component: {}", e))
            })?;

        let plugin = instance.get_func(&mut self.store, "run").ok_or(
            WasmRuntimeError::ComponentExecution("Failed to get plugin entry point".to_string()),
        )?;

        let args = [Val::List(
            options
                .iter()
                .map(plugin_option_to_val)
                .collect::<Result<Vec<Val>, _>>()?,
        )];
        let mut result = [Val::S32(0)];
        plugin
            .call_async(&mut self.store, &args, &mut result)
            .await
            .map_err(|e| {
                WasmRuntimeError::ComponentExecution(format!(
                    "Cannot call component function: {}",
                    e
                ))
            })
    }

    fn find_file(&self, search_filename: String) -> Option<PathBuf> {
        self.paths
            .iter()
            .filter(|path| path.is_dir())
            .find(|path| {
                let Ok(entries) = path.read_dir() else {
                    return false;
                };
                for entry in entries {
                    match entry {
                        Ok(entry) if entry.path().is_file() => {
                            if let Some(filename) = entry.file_name().to_str() {
                                if search_filename == filename {
                                    return true;
                                }
                            }
                        }
                        _ => (),
                    }
                }
                false
            })
            .map(|p| p.join(search_filename))
    }
}

impl Default for WasmRuntime<'_> {
    fn default() -> Self {
        let mut config = Config::new();
        config.async_support(true);
        config.wasm_component_model(true);

        WasmRuntime::new(config).expect("Create default engine")
    }
}

fn plugin_option_to_val(plugin_option: &PluginOption) -> Result<Val, WasmRuntimeError> {
    Ok(Val::Record(vec![
        ("key".to_owned(), identifier_to_val(plugin_option.name())?),
        (
            "value".to_owned(),
            Val::Variant(
                "string-t".to_owned(),
                Some(Box::new(value_to_val(plugin_option.value())?)),
            ),
        ),
    ]))
}

fn identifier_to_val(identifier: &Identifier) -> Result<Val, WasmRuntimeError> {
    Ok(Val::String(identifier.as_str().to_owned()))
}

fn value_to_val(value: &Value) -> Result<Val, WasmRuntimeError> {
    Ok(Val::String(value.into()))
}
