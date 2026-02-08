# AGENTS.md — Bareops

Bareops is an Infrastructure-as-Code CLI tool written in Rust. It parses a custom DSL
(`.bl` taskbook files) and executes tasks via WASM component-model plugins. The project
is a Cargo workspace hosted on Codeberg.

## Build / Lint / Test Commands

```sh
# Build the entire workspace
cargo build

# Run all tests
cargo test

# Run tests for a single crate
cargo test -p bareops-lang
cargo test -p bareops            # bareops-main (package name is "bareops")

# Run a single test by name (substring match)
cargo test -p bareops-lang test_minimal_task
cargo test -p bareops test_find_file

# Format check (CI enforced — no rustfmt.toml, uses defaults)
cargo fmt --check

# Lint (CI enforced — no clippy.toml, uses defaults)
cargo clippy

# Build WASM example plugins (requires cargo-component and just)
cargo install cargo-component
just build-examples

# Dependency audit (cargo-deny, configured in deny.toml)
cargo deny check
```

## Workspace Layout

```
bareops-main/   (bin+lib, package name "bareops") — CLI, WASM runtime, error types
bareops-lang/   (lib) — DSL pest grammar + AST types
bareops-plugin-api/ (lib) — WIT interface + wit-bindgen re-exports for plugins
examples/       — hello-world-plugin, azure-plugin (cdylib → WASM components)
```

## Code Style

### Formatting
- Default `rustfmt` settings — no `rustfmt.toml` exists. CI runs `cargo fmt --check`.
- 4-space indentation, no tabs.
- Trailing commas on the last field/item in multi-line structs, vecs, and argument lists.
- Opening brace on the same line as the declaration.
- Long method chains break with one call per line, indented.

### Imports
- All `use` statements in a single contiguous block at the top of each file (no blank-line
  grouping between std / external / internal).
- Use `crate::` for intra-crate imports (`use crate::error::BareopsError;`).
- Prefer separate `use` lines per submodule path over deeply nested trees:
  ```rust
  use wasmtime::component::{Component, Linker, ResourceTable, Val};
  use wasmtime::{Config, Engine, Store};
  ```
- Curly braces for multi-item imports: `use std::path::{Path, PathBuf};`
- Avoid glob imports (`*`) except in plugin crates using generated bindings.

### Naming
- Types / Structs / Enums: `UpperCamelCase` (`WasmRuntime`, `ParseFailure`, `TaskBuilder`).
- Acronyms stay uppercase in type names: `DSLParser`, `IO` variant.
- Functions / methods / variables / fields: `snake_case`.
- Modules and files: `snake_case` matching the module name (`wasm_runtime.rs`).
- Crate names: kebab-case in Cargo.toml (`bareops-lang`), underscore in Rust (`bareops_lang`).
- Test functions: `test_` prefix + descriptive name; suffix `_fail` for negative cases.

### Module Organization
- File-per-module pattern (no `mod.rs`). Each module is a single `.rs` file alongside
  `lib.rs` or `main.rs`.
- Private modules (`mod foo;`) with selective `pub use` re-exports in `lib.rs` to form
  the public API.
- Module declarations come before `use` statements in `lib.rs`/`main.rs`.

### Types
- Newtype pattern for domain primitives: `pub struct Identifier(pub String);`
- Builder pattern (manual, not derive-based) for complex construction: `TaskBuilder`,
  `PluginBuilder`. Setters take `&mut self` → `&Self`; `build()` consumes `self`.
- No type aliases used.
- Common derives: `Debug` on all types; `PartialEq, Eq` on AST value types;
  `Error` (thiserror) on error enums; `Diagnostic` (miette) on user-facing errors.
- `Clone`, `Copy`, `Default` are not derived — `Default` is implemented manually
  when needed (e.g., `WasmRuntime`).

### Error Handling
- Two-crate combo: `thiserror` for `Error` derive, `miette` for diagnostic reporting.
- Top-level error: `BareopsError` derives `Error, Debug, Diagnostic` — all variants
  wrap `String`.
- Module-level errors (e.g., `WasmRuntimeError`): derive `Error, Debug` only (no
  `Diagnostic`).
- Rich parse errors: `ParseFailure` struct carries `#[source_code]` (NamedSource),
  `#[label]` (SourceSpan), and `#[help]` (String) for miette diagnostics.
- Error conversion: manual `From` impls, not `#[from]`. Errors are converted to
  strings early via `.to_string()`.
- Use `.map_err(|e| ...)` with closures to map lower-level errors to domain errors.
- Use `?` operator throughout; `Err(...)? ` for inline error creation in match arms.
- `main()` returns `miette::Result<()>`; use `.into_diagnostic()?` to convert
  non-miette errors.
- Library functions return `Result<T, CustomError>`.

### Documentation & Comments
- No doc comments (`///`) are used in the current codebase.
- Inline `//` comments for TODOs and brief explanations only.
- Comments use `//` with a single space after the slashes.

### Testing
- All tests are inline: `#[cfg(test)] mod tests { ... }` at the bottom of each file.
- Tests import with `use super::*;` plus any additional specific imports.
- Raw string literals (`r#"..."#`) for multi-line DSL test input.
- All assertions include a custom message:
  ```rust
  assert_eq!(tasks.len(), 1, "Expected {} task", 1);
  assert!(tasks.is_err(), "Expected failure");
  ```
- No external test frameworks, fixtures, or shared test utilities.

### Key Dependencies
- **CLI**: clap v4.5
- **Async**: tokio v1.44 (multi-threaded)
- **WASM**: wasmtime v31.0 (Component Model), wit-bindgen v0.41
- **Parsing**: pest / pest_derive (PEG grammar in `dsl.pest`)
- **Errors**: thiserror v2.0, miette v7.5 (fancy feature)
- **Logging**: log v0.4 + env_logger v0.11

### CI Checks (Forgejo Actions)
All of these run on PRs and pushes to `main`:
1. `cargo test` — unit tests
2. `cargo fmt --check` — formatting
3. `cargo clippy` — linting
4. `cargo build` — CLI build
5. `just build-examples` — WASM plugin builds (requires cargo-component)

### Things to Watch
- No `#[allow(...)]` or `#[deny(...)]` attributes — keep it that way; let default
  clippy and rustc warnings surface.
- `cargo-deny` (deny.toml) is configured for supply-chain auditing.
- The `bareops-conn` and `bareops-integration-test` directories are not workspace
  members yet (future/placeholder crates).
- CODEOWNERS: `@frode_0xa @niveau0`
