# chore: remove Elixir tooling (targets 2.2.4)

## Summary

- Deletes `tooling/elixir/` (120 files — the entire Elixir reference implementation)
- Removes `elixir:test` and `rust:tooling:parity` CI jobs; both existed solely to build/test the Elixir code
- Removes `.elixir-base` CI template and all `tooling/elixir/` path references from Trivy and cargo-deny triggers
- Updates `agf/dist/release-manifest.yaml`, `agf/dist/README.md`, all `agf/docs/contracts/` pages, `agf/docs/site/reference/tooling-conformance.md`, and `agf/ADOPTING.md` to reflect the Rust `agov` binary as the sole implementation

The Rust tooling has been the production implementation since 2.1.x. The Elixir code was superseded; keeping it caused CI noise (a parity job that ran on a hexpm/elixir image and compiled Rust from source), cache pollution, and documentation confusion about which implementation was authoritative.

Historical contract documents (`binding-contract.md`, `contract-stability.md`, `integration-contract.md`) retain their `mix agov.*` references — those entries document the v1.x/E3 migration phases and are labelled as such. Rewriting them would obscure the record of how the implementation evolved.

## Test plan

- [ ] `agov dist validate` passes on the updated `release-manifest.yaml`
- [ ] CI pipeline: all remaining jobs green (no Elixir/parity jobs present)
- [ ] `dependency:cve-scan` passes without the removed `tooling/elixir/` skip-dirs
- [ ] MkDocs strict build passes (no broken internal links from updated docs)
