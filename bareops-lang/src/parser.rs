use crate::ast::{Identifier, PluginBuilder, PluginOption, Tag, Task, TaskBuilder, Value};
use miette::{Diagnostic, NamedSource, Result, SourceSpan};
use pest::error::{Error, InputLocation, LineColLocation};
use pest::iterators::Pairs;
use pest::Parser;
use pest_derive::Parser;
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
#[error("Syntax error")]
pub struct ParseFailure {
    #[source_code]
    src: NamedSource<String>,

    #[label("This bit here")]
    bad_code: SourceSpan,

    #[help]
    help: String,
}


#[derive(Parser)]
#[grammar = "dsl.pest"]
struct DSLParser;

fn create_task(
    pairs: Pairs<Rule>,
    task_builder: &mut TaskBuilder,
    source: &NamedSource<String>,
) -> Result<()> {
    let mut tags = Vec::new();
    for pair in pairs {
        match pair.as_rule() {
            Rule::task_name => {
                task_builder.name(pair.into_inner().as_str().to_string());
            }
            Rule::task_tag => {
                tags.push(Tag::from(pair.into_inner().as_str()));
            }
            Rule::task_plugin => {
                let mut plugin_builder = PluginBuilder::new();
                create_plugin(pair.into_inner(), &mut plugin_builder, source)?;
                task_builder.plugin(plugin_builder.build());
            }
            _ => Err(ParseFailure {
                src: source.clone(),
                bad_code: (0, 0).into(),
                help: "Expected task part".to_string(),
            })?,
        };
    }
    task_builder.tags(tags);
    Ok(())
}

fn create_plugin(
    pairs: Pairs<Rule>,
    plugin_builder: &mut PluginBuilder,
    source: &NamedSource<String>,
) -> Result<()> {
    let mut options = Vec::new();
    for pair in pairs {
        match pair.as_rule() {
            Rule::ident => {
                plugin_builder.name(Identifier::new(pair.as_str()));
            }
            Rule::plugin_option => {
                options.push(create_option(pair.into_inner())?);
            }
            _ => Err(ParseFailure {
                src: source.clone(),
                bad_code: (0, 0).into(),
                help: "Expected plugin part".to_string(),
            })?,
        };
    }
    plugin_builder.options(options);
    Ok(())
}

fn create_option(pairs: Pairs<Rule>) -> Result<PluginOption> {
    let mut ident = None;
    let mut value = None;
    for pair in pairs {
        match pair.as_rule() {
            Rule::ident => {
                ident = Some(Identifier::new(pair.as_str()));
            }
            Rule::value => {
                value = Some(Value::new(pair.as_str()));
            }
            _ => Err(ParseFailure {
                src: NamedSource::new("DSL", pair.to_string()),
                bad_code: (0, 0).into(),
                help: "Expected option.".to_string(),
            })?,
        }
    }
    Ok(PluginOption::new(
        ident.expect("Option identifier"),
        value.expect("Value for option"),
    ))
}

fn build_ast(pairs: Pairs<Rule>, source: &NamedSource<String>) -> Result<Vec<Task>> {
    let mut tasks = vec![];
    if pairs.len() == 0 {
        return Ok(tasks);
    }
    for pair in pairs {
        let mut task_builder = TaskBuilder::new();
        match pair.as_rule() {
            Rule::task => {
                create_task(pair.into_inner(), &mut task_builder, source)?;
                tasks.push(task_builder.build());
            }
            Rule::EOI => (),
            _ => Err(ParseFailure {
                src: source.clone(),
                bad_code: (0, 0).into(),
                help: "Expected task".to_string(),
            })?,
        };
    }
    // TODO: are duplicate task names a problem? e.g. for hash maps? maybe add an internal identifier?
    Ok(tasks)
}

fn create_help(err: Error<Rule>) -> String {
    match err.line_col {
        LineColLocation::Pos((line, col)) => {
            format!("At line {}, column {}", line, col)
        }
        LineColLocation::Span((line1, col1), (line2, col2)) => format!(
            "From line {}, column {} to line {}, column {}",
            line1, col1, line2, col2
        ),
    }
}


pub fn parse(source: &NamedSource<String>) -> Result<Vec<Task>> {
    match DSLParser::parse(Rule::tasks, source.inner()) {
        Ok(pairs) => build_ast(pairs, source),
        Err(err) => {
            let parse_error = ParseFailure {
                src: source.clone(),
                bad_code: match err.location {
                    InputLocation::Pos(pos) => SourceSpan::from(pos),
                    InputLocation::Span((start, len)) => SourceSpan::from((start, len)),
                },
                help: create_help(err),
            };
            Err(parse_error.into())
        }
    }
}

#[cfg(test)]
mod tests {
    use log::error;
    use super::*;
    use crate::ast::{Identifier, PluginOption, Tag, Value};

    #[test]
    fn test_empty() {
        let input = r#"

    "#;
        let tasks = parse(&NamedSource::new("test", input.to_string())).unwrap();
        assert_eq!(tasks.len(), 0, "Expected {} tasks", 0);
    }

    #[test]
    fn test_minimal_task() {
        let input = r#"
    task "example" {
        minimal_plugin {
        }
    }"#;
        let tasks = parse(&NamedSource::new("test", input.to_string())).unwrap();
        assert_eq!(tasks.len(), 1, "Expected {} task", 1);
    }

    #[test]
    fn test_invalid_task() {
        let input = r#"
    task "example" {
    }"#;
        let tasks = parse(&NamedSource::new("test", input.to_string()));
        assert!(tasks.is_err(), "Expected failure");
        let err = tasks.unwrap_err();
        error!("{:?}", err);
    }

    #[test]
    fn test_task_with_simple_plugin() {
        let input = r#"
    task "example" {
        tags: [  "tag1" ,  "tag2" ]

        thePlugin  {
            option1:  "value1"
            option2: "value2"
        }
    }"#;

        let tasks = parse(&NamedSource::new("test", input.to_string())).unwrap();
        assert_eq!(tasks.len(), 1, "Expected {} task", 1);

        assert_eq!(tasks[0].name(), "example");
        assert!(
            tasks[0].tags().contains(&Tag::from("tag1")),
            "Expected tag1"
        );
        assert!(
            tasks[0].tags().contains(&Tag::from("tag2")),
            "Expected tag2"
        );
        assert_eq!(tasks[0].plugin().name(), &Identifier::from("thePlugin"));
        assert!(
            tasks[0].plugin().options().contains(&PluginOption::new(
                Identifier::from("option1"),
                Value::from("value1")
            )),
            "Expected option1:value1"
        );
        assert!(
            tasks[0].plugin().options().contains(&PluginOption::new(
                Identifier::from("option2"),
                Value::from("value2")
            )),
            "Expected option2:value2"
        );
    }

    #[test]
    fn test_multiple_simple_tasks() {
        let input = r#"
    task "example0" {
        tags: ["tag1",
          "tag2"]

        thePlugin  {
            option1:"value1"
            
            option2: "value2"
        }
    }
    task "example1" {
        tags:   ["tag1","tag2"]

        thePlugin {
            option1: "value1"
            option2: "value2"
            
        }
    }
    "#;

        let tasks = parse(&NamedSource::new("test", input.to_string())).unwrap();
        assert_eq!(tasks.len(), 2, "Expected {} task", 2);

        tasks.iter().enumerate().for_each(|(idx, task)| {
            assert_eq!(task.name(), &format!("example{}", idx));

            assert!(task.tags().contains(&Tag::from("tag1")), "Expected tag1");
            assert!(task.tags().contains(&Tag::from("tag2")), "Expected tag2");
            assert_eq!(task.plugin().name(), &Identifier::from("thePlugin"));
            assert!(
                task.plugin().options().contains(&PluginOption::new(
                    Identifier::from("option1"),
                    Value::from("value1")
                )),
                "Expected option1:value1"
            );
            assert!(
                task.plugin().options().contains(&PluginOption::new(
                    Identifier::from("option2"),
                    Value::from("value2")
                )),
                "Expected option2:value2"
            );
        });
    }
}
