fn main() {
    capnpc::CompilerCommand::new()
        .src_prefix("schema")
        .default_parent_module(vec!["rpc".into()])
        .file("schema/bareops.capnp")
        .run()
        .expect("schema compiler command");
}
