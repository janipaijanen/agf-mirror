<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Project artifact validation and indexing

The optional artifact registry lets projects supply their own JSON Schema and
ID/reference declarations. It validates structure, global ID uniqueness,
reference existence and declared dependency acyclicity. It does not authenticate
reviewers, approve gates, mint IDs, generate work packets or compare branches.
These commands provide structural evidence; no new governance-blocking CI rule
is introduced by enabling a registry.

## Commands and inputs

```sh
agov artifact validate --config agov-config.yaml
agov artifact validate --config agov-config.yaml --format json items/a.json
agov artifact index --config agov-config.yaml --format json
```

`validate` defaults to text; `index` defaults to JSON. Both accept `text` or
`json`, use the same engine and inspect **all configured sets**. Optional paths
on `validate` assert that those exact files belong to the registry; they do not
hide other files or bypass cross-file checks. Paths and `--config` are relative
to the AGOV project root. The existing AGOV root finder recognizes an ancestor
with `agf/schemas` and `tooling`; otherwise it uses the current directory. Run
from the project root when using a standalone binary in another layout.

The default config is a root `agov-config.yaml`; an explicit repository-relative
path can select a config elsewhere. No automatic search of coordination config
files occurs. Existing root keys such as `from_aliases` are ignored by the
artifact reader and retain their existing meaning. With no `artifact_sets`,
a readable config describes an empty registry. A missing config is an error.

## Configuration

```yaml
artifact_sets:
  - name: work-packages
    paths: ['items/*.json']
    schema: schemas/work-package.json
    id_pointers: ['/id']
    reference_pointers: ['/references', '/dependencies']
    dependency_pointers: ['/dependencies']
    uniqueness_scope: global
```

The config shape is described by `agf/schemas/artifact-config.schema.json`.
Set names must be nonempty and unique. Unknown artifact-set fields are errors.
Paths, schema and at least one ID pointer are required. Reference and dependency
lists default to empty. V1 supports only global uniqueness across every set;
namespace-specific uniqueness is not silently substituted. Selecting the same
ID-bearing file in two different sets therefore produces a duplicate-ID error.

Artifact documents may be JSON or YAML; the registered schema must be JSON,
using the existing Draft-07 validator. Schemas must be self-contained: local
fragment `$ref` values are supported, while external file/network references
are rejected. Artifact commands do not fetch a schema or execute a plugin.

Pointers use exact RFC 6901 syntax: `/nodes/0/id` selects one value, `~1` escapes
`/`, and `~0` escapes `~`. A `*` token is a literal property name, not array
expansion. List exact pointers for multiple nested records. ID pointers must
resolve to nonempty strings. A reference pointer resolves to one nonempty ID
string or an array of them; use an empty array to declare no references. Missing
configured pointers are errors, even when a project schema makes a field optional.

Each reference belongs to its nearest enclosing object with a declared ID.
Multiple IDs at that same enclosing level make reference ownership ambiguous
and are rejected. A dependency pointer must also be declared as a reference
pointer. Only those references create dependency edges; ordinary cross-references
may form loops. All targets must resolve uniquely, including targets in other sets.

## Scope and containment

Only configured local files are inputs; there is no branch union or remote fetch.
Globs match individual path components, with `**` as a complete recursive
component. Results are sorted and deduplicated within a set. Every pattern must
match a regular file; unmatched or unreadable selections make coverage incomplete.
Absolute paths, traversal, backslash/drive paths and symlink traversal are rejected,
including schema/config symlinks and intermediate directories. V1 rejects internal
symlinks as well as external ones. Each file is limited to 16 MiB and discovery
is bounded to 100,000 traversal states per pattern.

This is a local working-tree observation, not an atomic Git snapshot. Keep inputs
stable while it runs. Repeated references to the same path reuse captured bytes;
ordinary size/mtime changes detected during a read cause failure. The path checks
are not a claim of isolation against a hostile process racing filesystem mutation.
The required independent containment-security review must assess that operating
boundary before release acceptance.

## Result and failure semantics

JSON is deterministic for unchanged inputs and contains no generation timestamp:

- `schema_version`: `1`.
- `coverage`: `COMPLETE` or `INCOMPLETE` for the configured inventory.
- `files`: sorted registered file paths.
- `entries`: sorted ID rows with `id`, `kind`, `key`, `version`, `set`, `path`,
  exact ID `pointer`, SHA-256 `digest` of raw file bytes, and `status`.
- `edges`: source ID, target ID, dependency flag and source path/pointer.
- `problems`: stable-sorted code/path/pointer/message diagnostics.

`kind`, `key` and `version` come from the ID's enclosing object. Missing `kind`
uses the set name; missing key/version are null. Status is `valid` or `invalid`;
a file with findings is invalid, and incomplete inventory invalidates all returned
entries. Entries can remain present when their project schema fails so that
conflicting IDs and unresolved references are not silently concealed.

Exit 0 means complete input and no findings; exit 1 means configuration, document
or structural errors; exit 2 means unavailable/incomplete input and takes precedence.
A cycle is reported as a graph-level error, without claiming every downstream
node belongs to that cycle. No result supplies human approval or release authority.

MCP exposes `agov_artifact_validate` (`config`, `format`, optional `paths`) and
`agov_artifact_index` (`config`, `format`). These invoke the same CLI; stdout,
exit status and errors are preserved by the existing adapter. Neither writes an
output file. Rebuild and restart the MCP server to expose newly added tools.

The checked-in CLI fixtures include a full independent expected index, duplicate
IDs, dangling references, a dependency cycle and malformed config. Separate tests
exercise missing files, symlink containment and repeated byte-identical output.
