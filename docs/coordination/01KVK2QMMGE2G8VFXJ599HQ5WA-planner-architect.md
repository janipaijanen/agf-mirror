---
kind: handoff
from: planner-architect
to: coder-tester
id: 01KVK2QMMGE2G8VFXJ599HQ5WA
timestamp: 2026-06-20T17:54:52Z
immutable: true
references-to:
  - 01KVK1YT6KZNVN0WVFNV6728JT-coder-tester.md
topic: rust-agov-phase1-scaffold-api-review
requires-ack: true
artifacts_owned:
  - tooling/rust/Cargo.toml
  - tooling/rust/.cargo/config.toml
  - tooling/rust/crates/agov-core/Cargo.toml
  - tooling/rust/crates/agov-core/src/lib.rs
  - tooling/rust/crates/agov-core/src/yaml.rs
  - tooling/rust/crates/agov-core/src/schema.rs
  - tooling/rust/crates/agov-core/src/infosec.rs
  - tooling/rust/crates/agov-core/src/ulid_util.rs
  - tooling/rust/crates/agov-core/src/roi.rs
  - tooling/rust/crates/agov-core/src/output.rs
  - tooling/rust/crates/agov-coord/Cargo.toml
  - tooling/rust/crates/agov-coord/src/lib.rs
  - tooling/rust/crates/agov-coord/src/coord_validator.rs
  - tooling/rust/crates/agov-coord/src/plan_validator.rs
  - tooling/rust/crates/agov-coord/src/channel_reader.rs
  - tooling/rust/crates/agov-coord/src/handoff_tracker.rs
  - tooling/rust/crates/agov-validators/Cargo.toml
  - tooling/rust/crates/agov-validators/src/lib.rs
  - tooling/rust/crates/agov-validators/src/adoption.rs
  - tooling/rust/crates/agov-validators/src/agents.rs
  - tooling/rust/crates/agov-validators/src/learnings.rs
  - tooling/rust/crates/agov-validators/src/infosec.rs
  - tooling/rust/crates/agov-validators/src/contributions.rs
  - tooling/rust/crates/agov-validators/src/framework_config.rs
  - tooling/rust/crates/agov/Cargo.toml
  - tooling/rust/crates/agov/src/main.rs
  - tooling/rust/tests/parity/run.sh
  - .gitlab-ci.yml
handoff:
  next_agent: coder-tester
  status: ready
  task:
    - Review the scaffold API surface (public function signatures and types)
      before starting logic implementation. Confirm, amend, or block.
    - After API review is complete, begin Phase 1 feature-slice implementation
      per the agreed sequencing in 01KVK1YT6KZNVN0WVFNV6728JT.
  must_read:
    - tooling/rust/crates/agov-core/src/lib.rs
    - tooling/rust/crates/agov-core/src/yaml.rs
    - tooling/rust/crates/agov-core/src/schema.rs
    - tooling/rust/crates/agov-core/src/output.rs
    - tooling/rust/crates/agov-core/src/roi.rs
    - tooling/rust/crates/agov-coord/src/coord_validator.rs
    - tooling/rust/crates/agov-coord/src/plan_validator.rs
    - tooling/rust/crates/agov-validators/src/lib.rs
    - tooling/rust/crates/agov/src/main.rs
  do_not_touch:
    - tooling/elixir/
    - agf/
---

# Scaffold API review request — rust-agov-phase1

The Phase 1 workspace scaffold is on `feat/rust-agov-phase1`. All logic stubs
use `todo!()`. Please review the public API surface before writing logic.

## Summary of design decisions

### Common types (`agov-core/src/lib.rs`)

```rust
pub enum Severity { Blocking, Advisory }

pub struct Finding {
    pub severity: Severity,
    pub message: String,
    pub location: Option<String>,
}
```

`Blocking` maps to `"severity: blocking"` in the Elixir output; `Advisory`
maps to `"severity: advisory"`. The `location` field corresponds to a JSON
pointer (`"#/advisory_consultation"`) for schema findings, or a human-readable
path token for semantic findings.

### YAML (`agov-core/src/yaml.rs`)

```rust
pub fn extract_frontmatter(content: &str) -> Result<String, YamlError>
pub fn parse_yaml<T: DeserializeOwned>(yaml: &str) -> Result<T, YamlError>
pub fn parse_frontmatter<T: DeserializeOwned>(content: &str) -> Result<T, YamlError>
```

Mirrors `Agov.CoordValidator.extract_frontmatter/1`.

### Schema (`agov-core/src/schema.rs`)

```rust
pub struct CompiledSchema(jsonschema::Validator);   // opaque wrapper
pub struct SchemaFinding { pub message: String, pub pointer: String }

pub fn load_and_compile(path: &Path) -> Result<CompiledSchema, SchemaError>
pub fn validate(schema: &CompiledSchema, data: &serde_json::Value) -> Vec<SchemaFinding>
```

Draft-07, no remote refs, no async. `SchemaFinding` maps to the `(message,
json_pointer)` tuple from `ExJsonSchema.Validator.validate/2`.

### Output (`agov-core/src/output.rs`)

Two format families derived directly from the Elixir source:

**coord/plan validate** (from `cli.ex`):
```rust
pub fn coord_ok(path: &str)                                         // "OK  severity: blocking {path}"
pub fn coord_warn(path: &str, msg: &str)                           // "WARN severity: advisory {path}: {msg}"
pub fn coord_err(path: &str, msg: &str)                            // "ERR severity: blocking {path}: {msg}" → stderr
pub fn coord_err_schema(path: &str, errors: &[(String, String)])   // multiline schema errors → stderr
```

**validate sub-commands** (from `ValidateReporter`):
```rust
pub fn validate_ok(label: &str)
pub fn validate_warn(label: &str, location: &str, message: &str)
pub fn validate_err(label: &str, location: &str, message: &str)
pub fn validate_err_label(label: &str)
```

### ROI logging (`agov-core/src/roi.rs`)

```rust
pub struct RoiRecord { /* fields per plan §8.2 */ }
pub fn log_path() -> Option<PathBuf>   // None when disabled
pub fn try_append(record: &RoiRecord)  // non-fatal, side-channel only
```

### Coord validator (`agov-coord/src/coord_validator.rs`)

```rust
pub struct CoordMessage { pub raw: serde_json::Value }

pub fn validate_file(path: &Path) -> Result<CoordMessage, Vec<(String, String)>>
pub fn advisory_findings(msg: &CoordMessage) -> Vec<String>
```

Error type `Vec<(String, String)>` = `(message, json_pointer)` pairs, matching
Elixir's `ExJsonSchema` tuple format and the `cli.ex` rendering loop.

### Plan validator (`agov-coord/src/plan_validator.rs`)

```rust
pub struct PlanDocument { pub raw: serde_json::Value }
pub fn validate_file(path: &Path) -> Result<PlanDocument, Vec<(String, String)>>
```

### Collection result (`agov-validators/src/lib.rs`)

```rust
pub struct CollectionResult { pub ok_files: Vec<String>, pub file_errors: Vec<FileError> }
pub struct FileError { pub file: String, pub findings: Vec<Finding> }
```

Used by `agents::validate_all`, `learnings::validate_all`, `infosec::validate_all`,
`contributions::validate_all`. Mirrors `{:ok, files}` / `{:error, file_errors}`.

### Binary CLI (`agov/src/main.rs`)

Three top-level subcommands: `coord`, `plan`, `validate`. Phase 1 commands
only; Phase 2 stubs marked in comments.

## Questions for Codex

1. Is `CoordMessage { pub raw: serde_json::Value }` sufficient, or should
   specific fields be typed (e.g. `kind`, `from`, `to`, `id`) to avoid
   repeated `.get("kind")` calls throughout the implementation?

2. The `CompiledSchema` wrapper hides `jsonschema::Validator`. Is this the
   right tradeoff, or should Codex own the schema crate entirely and expose
   a simpler `validate_file(path, schema_name)` function instead?

3. Cargo.lock is not committed — cargo is not installed locally. Codex
   should run `cargo generate-lockfile` and commit it in the first
   implementation commit.

4. The parity harness skeleton (`tests/parity/run.sh`) expects Codex to
   add `run_case` invocations and generate reference files as each command
   is implemented. Confirm this is understood.
