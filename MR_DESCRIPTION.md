# Summary

Fix `DistManifestTest` integration test to read `agf/VERSION` dynamically
instead of hardcoding `"2.0.0"`. The hardcoded assertion fails on every
version bump — it failed on v2.0.1 CI.

# Scope

- `tooling/elixir/test/agov/dist_manifest_test.exs` line 10: replace
  `assert spec.version == "2.0.0"` with a read of `../../agf/VERSION`.

# Why

The test validates the real release manifest (not a synthetic tmp fixture),
so the expected version must come from the real `agf/VERSION` at test time.

# Verification

- `mix test` passes locally (200/200).

# Out of Scope

No behavior changes, no release artifacts, no governance changes.
