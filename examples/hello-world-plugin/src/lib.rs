use bareops_plugin_api::bindings::export;
use bareops_plugin_api::Plugin;

// Define a new struct for your plugin. Name doesn't matter
struct HelloWorldPlugin;

// impl the trait provided by the Plugin API. rust-analyzer should tell you that it
// expects a function that matches the shape of the function defined in the wit file
impl Plugin for HelloWorldPlugin {
    fn run(a: i32, b: i32) -> i32 {
        println!("Hello World, args were {},{}", a, b);
        1
    }
}
// Call the export! macro with the struct for our plugin, but since the bindings
// for wit are defined in plugin_api, we need to add `with_types_in` as the second arg
// and the path to the bindings as the third
export!(HelloWorldPlugin with_types_in bareops_plugin_api::bindings);
