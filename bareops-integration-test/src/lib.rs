use std::process::Command;

use miette::NamedSource;

pub fn test_with_shell_script(name: &str, task_book: &str, shell_script: &str) {
    let source = NamedSource::new(name, task_book.to_string());
    let tasks = bareops_lang::parse(&source).unwrap();
    dbg!(tasks);
    Command::new("scp");
}

#[macro_export]
macro_rules! test {
    ($name:ident, $taskbook:expr, sh $testscript:expr) => {
        use bareops_integration_test::test_with_shell_script;
        #[test]
        fn $name() {
            let name = stringify!($name);
            let task_book = $taskbook;
            let test_script = $testscript;

            test_with_shell_script(name, task_book, test_script);
        }
    };
}
