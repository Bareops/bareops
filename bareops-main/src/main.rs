use miette::NamedSource;

fn main() {
    let source = NamedSource::new("demo", "...".to_string());
    let _tasks = match bareops_lang::parse(&source) {
        Err(e) => {
            eprintln!("{:?}", e);
            return;
        }
        Ok(tasks) => tasks,
    };
}
