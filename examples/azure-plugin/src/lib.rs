use bareops_plugin_api::bindings::*;
use bareops_plugin_api::Plugin;

struct AzurePlugin;

impl Plugin for AzurePlugin {
    fn run(options: Vec<PluginOption>) -> Result<PluginType, String> {
        println!("Azure plugin, args were {:?}", options);
        Ok(PluginType::Int32T(1))
    }
}
// Call the export! macro with the struct for our plugin, but since the bindings
// for wit are defined in plugin_api, we need to add `with_types_in` as the second arg
// and the path to the bindings as the third
export!(AzurePlugin with_types_in bareops_plugin_api::bindings);
