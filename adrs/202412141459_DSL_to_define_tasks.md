# DSL to define tasks
[//]: # (Decision record template by Michael Nygard)

## Status
[//]: # (What is the status, such as proposed, accepted, rejected, deprecated, superseded, etc.?)
proposed

## Context
[//]: # (What is the issue that we're seeing that is motivating this decision or change?)
We want a more structured and type safe DSL as currently know from many other IaC tools.

## Decision
[//]: # (What is the change that we're proposing and/or doing?)
Like many infrastructure as code tools, we want to create an own DSL (why not?) to define
tasks that should be executable on localhost and remote target system by our engine.

## Consequences
[//]: # (What becomes easier or more difficult to do because of this change?)
We need a DSL parser. We need plugins to extend common IDEs to write in our DSL 
with syntax support.
