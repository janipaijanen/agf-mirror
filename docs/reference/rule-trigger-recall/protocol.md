# Rule-trigger recall: manual protocol

Foundry-local. Plan `20261023T140050Z-rule-trigger-recall-corpus`, Gate 1 record
`20261023T144257Z-governor-reviewer-zc0p`. This page is a protocol, not a rule; it grants no
authority. The scenarios are in `corpus.yaml`; result records follow `result.schema.json`.

## What each mode measures

- **Route selection** (default): `claude -p` with no tools. It asks which rule file the agent
  would read first. It tests whether the always-loaded trigger text points at the right file.
  It does **not** show that the agent opens the file. Say "route selection" in every claim.
- **Actual open**: only the Read tool is enabled; the first Read target is recorded from
  `--output-format stream-json`. Only this mode supports a statement that the agent opens a rule.

## Fixed conditions

**Restrict the tool set with `--tools`, not with a prompt sentence or `--allowedTools`.** When the
session runs with permissions bypassed, `--allowedTools` only pre-approves tools; it does not
remove the others, and "do not use tools" is only an instruction. In the first run of this
protocol one trial in a repository working copy wrote a stray coordination record
(`claude-haiku-5-5`, role build-engineer) under `docs/coordination/` despite that
instruction. Run every trial in a throwaway worktree, use `--tools ""` (route selection) or
`--tools Read` (actual open), and check `git status --short` is empty after each run; delete and
report anything untracked that appears, and never publish it.

Record per run: client version (`claude --version`), model id, the exact head SHA of the working
copy, and the date. Use a clean session per trial (`claude -p`, no resume) in a throwaway worktree
at the head under test. Do not mix client versions or models inside one comparison.

## Route-selection trial

```
claude -p "<prompt>. Answer with ONLY the repo path of the single rule file you must read first, or NONE. No other text." --model <id> --tools "" < /dev/null
```

Take the first line of output as `answer`. Positive kinds (`direct`, `paraphrase`,
`near-boundary`) pass when `answer` is in the scenario's `accept`. A `negative` scenario passes
when `answer` is **not** in `accept`.

## Actual-open trial

```
claude -p "<prompt>. Read the single rule file you must read first." --model <id> --tools Read --output-format stream-json --verbose < /dev/null
```

`first_read_target` is the `file_path` of the first `Read` tool_use in the stream, or `null` if
there was none (which fails a positive trial). Positive and negative pass rules are as above, using
`first_read_target` in place of `answer`. Use a read-only working copy.

## Gates

- **Smoke gate** (after any trigger edit touching a covered rule): zero misses on every `high`
  scenario at N=3 per scenario. It is a regression screen, not a statistical claim. Acceptance of
  an implementation under this plan is smoke-only.
- **Actual-open smoke (the fixed set)**: 5 rules (release-via-merge-request, deputy-governor, merge-request-workflow,
  immutable-tags, coordination-challenge-response) x 3 scenarios (direct, paraphrase, negative) x 3
  trials x 1 model = 45 calls.
- **Claim gate** (only if the governor requests it): unit = one (rule, model) pair. Positive recall
  counts `direct`, `paraphrase` and `near-boundary` trials; the false-trigger rate counts `negative`
  trials; they are never pooled. The 95% Wilson lower bound must reach 0.95: with zero failures that
  needs n >= 73 trials per unit (73/73 gives 0.9500; 72/72 gives 0.9493), one failure n >= 110, two 142,
  three 173. Cost with zero failures: 13 rules x 73 x 2 models x 2 (positive and negative) = 3,796 paid
  calls; one allowed failure 5,720. Never run paid model calls in blocking CI.

## Hold-out

Scenarios with `holdout: true` are authored by coder-tester and are not read when editing trigger
lines (anti-overfitting). Whoever edits a trigger line uses only `holdout: false` scenarios.

## Result files and checking them

Write one JSON object per line to a dated local file, for example
`tmp/rule-trigger-recall-<date>-<head8>.jsonl`. The corpus and this protocol are committed;
raw result files are MR artifacts or dated local evidence.

**Draft-07 validation (the check that counts).** Validate every record against
`result.schema.json` with the repository's own vendored validator: the Rust crate `jsonschema`
**0.18.3** (see `tooling/rust/Cargo.lock`) driven by `agov_core::schema` in Draft-07 mode, with no
remote references. No extra dependency is installed. Put this throwaway example, which is not committed, at
`tooling/rust/crates/agov-core/examples/validate_jsonl.rs` and run
`cargo run -q -p agov-core --example validate_jsonl -- <schema.json> <results.jsonl>`; it prints
each finding and `valid=N invalid=M`, exit 0 only when M = 0:

```rust
use std::path::Path;
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let schema = agov_core::schema::load_and_compile(Path::new(&a[1])).expect("compile schema");
    let (mut ok, mut bad) = (0, 0);
    for (i, line) in std::fs::read_to_string(&a[2]).expect("read").lines().enumerate() {
        if line.trim().is_empty() { continue; }
        let v: serde_json::Value = serde_json::from_str(line).expect("json");
        let f = agov_core::schema::validate(&schema, &v);
        if f.is_empty() { ok += 1; } else { bad += 1; for x in f { println!("line {}: {} {}", i + 1, x.pointer, x.message); } }
    }
    println!("valid={ok} invalid={bad}");
    std::process::exit(if bad == 0 { 0 } else { 1 });
}
```

Keep the malformed-control evidence with every run: delete `first_read_target` from actual-open
records, set `tools_enabled` to `["Read","Bash"]`, add `first_read_target` to a route-selection
record, and set a bad `head_sha` and `kind`; each control must be rejected.

**Structural preflight (not a substitute).** `jq` is a quick sanity filter only; empty output
means the structure looks right:

```
jq -c '
  def req: ["scenario_id","rule","kind","model","client_version","head_sha","measurement_mode","tools_enabled","answer","pass","date_utc"];
  select(
    ((req - keys) | length > 0)
    or ((.measurement_mode | IN("route-selection","actual-open")) | not)
    or ((.kind | IN("direct","paraphrase","near-boundary","negative")) | not)
    or ((.head_sha | test("^[0-9a-f]{40}$")) | not)
    or (.measurement_mode == "actual-open" and ((has("first_read_target") | not) or (.tools_enabled != ["Read"])))
    or (.measurement_mode == "route-selection" and (has("first_read_target") or ((.tools_enabled | length) > 0)))
  )' results.jsonl
```

## Limits

Results from different client versions or model versions are not comparable without a fresh
baseline. A small sample is a smoke result, never a rate. Scenario prompts are synthetic and contain
no credentials.
