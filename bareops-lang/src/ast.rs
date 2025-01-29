#[derive(Debug)]
pub struct Task {
    name: String,
    tags: Vec<Tag>,
    plugin: Plugin,
}

impl Task {
    fn new(name: &str, tags: Vec<Tag>, plugin: Plugin) -> Self {
        Task {
            name: name.to_owned(),
            tags,
            plugin,
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn tags(&self) -> &Vec<Tag> {
        &self.tags
    }
    pub fn plugin(&self) -> &Plugin {
        &self.plugin
    }
}

pub struct TaskBuilder {
    name: Option<String>,
    tags: Option<Vec<Tag>>,
    plugin: Option<Plugin>,
}

impl TaskBuilder {
    pub fn new() -> Self {
        TaskBuilder {
            name: None,
            tags: None,
            plugin: None,
        }
    }
    pub fn name(&mut self, name: String) -> &Self {
        self.name = Some(name);
        self
    }

    pub fn tags(&mut self, tags: Vec<Tag>) -> &Self {
        self.tags = Some(tags);
        self
    }

    pub fn plugin(&mut self, plugin: Plugin) -> &Self {
        self.plugin = Some(plugin);
        self
    }

    pub fn build(self) -> Task {
        Task::new(
            self.name.as_deref().unwrap_or("unknown"),
            self.tags.unwrap_or_default(),
            self.plugin
                .unwrap_or(Plugin::new(Identifier("unknown".to_string()), Vec::new())),
        )
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Tag {
    name: String,
}

impl Tag {
    fn new(name: &str) -> Self {
        Tag {
            name: name.to_owned(),
        }
    }
}

impl AsRef<str> for Tag {
    fn as_ref(&self) -> &str {
        self.name.as_str()
    }
}

impl From<&str> for Tag {
    fn from(value: &str) -> Self {
        Tag::new(value)
    }
}

#[derive(Debug)]
pub struct Plugin {
    name: Identifier,
    options: Vec<PluginOption>,
}

impl Plugin {
    pub fn new(name: Identifier, options: Vec<PluginOption>) -> Self {
        Plugin { name, options }
    }

    pub fn name(&self) -> &Identifier {
        &self.name
    }

    pub fn options(&self) -> &Vec<PluginOption> {
        &self.options
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct PluginOption {
    name: Identifier,
    value: Value,
}

impl PluginOption {
    pub fn new(name: Identifier, value: Value) -> Self {
        PluginOption { name, value }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Identifier(String);

impl Identifier {
    pub fn new(name: &str) -> Self {
        Identifier(name.to_owned())
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl From<&str> for Identifier {
    fn from(value: &str) -> Self {
        Identifier::new(value)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Value(String);

impl Value {
    pub fn new(value: &str) -> Self {
        Value(value.to_owned())
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Value::new(value)
    }
}

pub struct PluginBuilder {
    name: Option<Identifier>,
    options: Option<Vec<PluginOption>>,
}

impl PluginBuilder {
    pub fn new() -> Self {
        PluginBuilder {
            name: None,
            options: None,
        }
    }
    pub fn name(&mut self, name: Identifier) -> &Self {
        self.name = Some(name);
        self
    }

    pub fn options(&mut self, options: Vec<PluginOption>) -> &Self {
        self.options = Some(options);
        self
    }

    pub fn build(self) -> Plugin {
        Plugin::new(
            self.name.unwrap_or(Identifier::new("unknown")),
            self.options.unwrap_or_default(),
        )
    }
}
