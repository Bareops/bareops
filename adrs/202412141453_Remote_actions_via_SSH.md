# Remote actions via SSH
[//]: # (Decision record template by Michael Nygard)

## Status
[//]: # (What is the status, such as proposed, accepted, rejected, deprecated, superseded, etc.?)
proposed

## Context
[//]: # (What is the issue that we're seeing that is motivating this decision or change?)
We don't want to reinvent the wheel and don't want to fiddle with network encryption.

## Decision
[//]: # (What is the change that we're proposing and/or doing?)
We want to use SSH as the mechanism to executed remote actions on target systems.

## Consequences
[//]: # (What becomes easier or more difficult to do because of this change?)
On the target systems, the host executable and all plugins that need to be used, must be 
transferred to the target hosts. Therefore, since WASM is executed in a sandbox, only
the host executable must match the target architecture.

All target machines need remote access port from a ssh daemon.
