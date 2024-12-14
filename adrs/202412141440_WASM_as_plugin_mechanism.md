# WASM as plugin mechanism
[//]: # (Decision record template by Michael Nygard)

## Status
[//]: # (What is the status, such as proposed, accepted, rejected, deprecated, superseded, etc.?)
proposed

## Context
[//]: # (What is the issue that we're seeing that is motivating this decision or change?)
We need a mechanism to easily extends the core component of our software without recompiling
and linking.

## Decision
[//]: # (What is the change that we're proposing and/or doing?)
We want to use WASM as the plugin mechanism for Rust.

## Consequences
[//]: # (What becomes easier or more difficult to do because of this change?)
Due to the nature of the current sandbox behavior this should protect against introducing 
of security issues within plugins. Additionally, the host can write audit logs for all 
restricted actions.