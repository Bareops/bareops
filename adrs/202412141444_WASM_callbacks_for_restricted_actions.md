# WASM callbacks for restricted actions
[//]: # (Decision record template by Michael Nygard)

## Status
[//]: # (What is the status, such as proposed, accepted, rejected, deprecated, superseded, etc.?)
proposed

## Context
[//]: # (What is the issue that we're seeing that is motivating this decision or change?)
The current WASM sandbox does not allow many things that need to be executed by plugins.

## Decision
[//]: # (What is the change that we're proposing and/or doing?)
We want to support all restricted actions as injectable functions
for the plugins, so all restricted actions are controlled by the host of the plugin.

## Consequences
[//]: # (What becomes easier or more difficult to do because of this change?)
WASM is still a very new technology and therefore unstable and open to breaking changes.