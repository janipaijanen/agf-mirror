---
kind: handoff
from: planner-architect
to: governor-reviewer
id: 20261101T034648Z-planner-architect-kb6g
timestamp: 2026-11-01T03:46:48Z
model: claude
immutable: true
references-to:
  - 20261101T032933Z-governor-reviewer-5rqt.md
  - 20261101T031520Z-planner-architect-as8p.md
topic: s2-main-integrated-rebind-5rqt
requires-ack: false
artifacts_owned: []
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review the S2 integration of current main into MR 3471 and the rebound package metadata (matrix and hash list only). Then the existing operator verifies the ZIP and bindings before any run decision. I authorize and run nothing.
  must_read:
    - docs/coordination/20261101T031520Z-planner-architect-as8p.md
  do_not_touch:
    - agf/docs/site/stylesheets/agf.css
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# S2 integrated with current main; package rebound (5rqt)

Per 5rqt. I merged current `main` into the existing MR !3471 with an ordinary merge and a normal push. I did not force-push, rebase or open a new MR. No browser, server, real unit, capture or upload was started.

## Integration facts

| Item | Value |
|---|---|
| New head of !3471 | `cf309782e94afd38b2acbe8a3c2c1a96fc005220` (merge commit) |
| First parent | `5df0ebea7740392980e3fdd03949355e6b225b45` (previous S2 head) |
| Second parent | `37828e4ac725b806ef8d8bd22f338badae546723` (`main` with S1 merged) |
| Remote branch head | equals the new head (checked with `git ls-remote`) |
| Pipeline | 3013528423, `merge_request_event`, all eight jobs success |
| HTML job | 17639326311 (`site-html-build`) |
| Artifact ZIP SHA-256 | `4e11043dc1da239a3a741e930de198ec4571bd910900008d4aa64b6294e3d3a0` (downloaded with `glab api`, 1899477 bytes) |
| `agf.css` blob in the head | `31864b3f27a7a2703cd6da06467797689ae0e7f7` (unchanged; the artifact `stylesheets/agf.css` is byte-identical to the one in the previous artifact) |

The aggregate diff of the head against `main` lists exactly four paths: `agf/docs/site/stylesheets/agf.css`, the S2 plan, the S2 report and the S2 projection manifest. The S2 plan still validates in CI (`agov:governance:plan-collision-gate` and `agov:governance:validate` are success).

## Artifact comparison against the previous S2 artifact (job 17638827528)

I compared the two extracted artifacts. This is my check; the existing operator verifies independently.

- The `<article>` region of the four real pages bound by the matrix is byte-identical in both artifacts (`docs/adopt/tooling-and-tokens/` for formula, exports and report-command, and `reference/review-findings/` for the long row). The selectors and full-text hashes in the matrix therefore bind to the same bytes as before. I did not re-evaluate the selectors in a browser.
- Of 52 pages, 5 have a different article region: `docs/concepts/roles-and-gates/`, `docs/concepts/coordination-protocol/`, `docs/start/should-you-adopt/`, `ai-evaluation-dossier/` and `reference/release-info/`. None is a matrix page. Other pages differ only in the wrapper, which now carries the S1 notice.
- Code blocks: I counted `<div class="highlight"` over all 52 pages. The previous artifact has 174. This artifact has 176. The 174-block survey in the S2 report is a model run on the previous artifact. It stays historical. I did not rerun the model on this artifact.

## Package rebinding

Only four matrix input fields changed, and the internal hash list was regenerated. A line diff of the matrix shows exactly these four lines:

| Matrix field | Before | After |
|---|---|---|
| `head` | `5df0ebea7740392980e3fdd03949355e6b225b45` | `cf309782e94afd38b2acbe8a3c2c1a96fc005220` |
| `ciPipeline` | 3013413250 | 3013528423 |
| `htmlJob` | 17638827528 | 17639326311 |
| `zipSha256` | `b76eab1f487ee248772752b7391239352bd2403c0ea0d233195c84ea1f883da3` | `4e11043dc1da239a3a741e930de198ec4571bd910900008d4aa64b6294e3d3a0` |

The three A3 helpers, the S1 static server, the observer, the driver, the stepped check, the generator, the launcher, `real-cases.json` and both test scripts are byte-identical to record as8p. The two files that changed are embedded below.

## Offline checks on the new artifact (no browser, no unit)

- `sha256sum -c s2-qa-hashes.sha256`: all 12 lines pass.
- `s2-qa-selftest.cjs` with the saved A3 witness projection: 39 of 39. This includes the generator on the new artifact (exactly one report-command match, full hash, entity and asset-depth checks).
- `s2-qa-run-selftest.sh` against the new artifact: 19 of 19.

The limits in as8p stand. Nothing ran in Chromium, the Tab limit of 150 is still an unmeasured estimate, and the real unit lifecycle is unproven beyond the S1 calibration.

## File inventory

| File | Bytes | SHA-256 | Status |
|---|---|---|---|
| `s2-qa-matrix.json` | 4599 | `361038a3d17bc1583c83d699918aaa888aef1b900215a091a894eb55512d913b` | CHANGED (embedded below) |
| `s2-qa-hashes.sha256` | 1006 | `c0e58ba465ebe2900fda3c76fa14d35cc18a3607d8ed61fbd8f1e27af10a8eac` | CHANGED (embedded below) |
| `code-measure.cjs` | 6837 | `3546a0396305d6b4e6085393de7f1aa4bd03887192f545866491560bd0a1a15e` | byte-identical to as8p |
| `code-proof.cjs` | 9405 | `2859dd21cac883249e9b2dd98f73e2dc80fc388ebad5168d03d1dc7949a846df` | byte-identical to as8p |
| `real-cases.json` | 1666 | `f22ff058196d788c635c5ace6b2d64e3b0bef3f4bf584307ca10198122d717ab` | byte-identical to as8p |
| `s1-qa-serve.cjs` | 4722 | `40db4af6b3e5c634ba1d121ca7daad018172805fb23ddd529ecbca9c1ca6ec06` | byte-identical to as8p |
| `s2-make-synthetic.cjs` | 3971 | `d3bc469540e27b3fbb608321029a89c68819941f2a619995dfb8d9d6344502bf` | byte-identical to as8p |
| `s2-qa-observe.cjs` | 9598 | `1b01ea7d8f6681e834dc77d371f5d2eb0ebe2f65db2f49be2489e19b00bfc56c` | byte-identical to as8p |
| `s2-qa-run-selftest.sh` | 6824 | `9bcc9d9c9e39c23a7f0d592ad6ee09b302ea9df75803dbb8b5c6557acd611cb5` | byte-identical to as8p |
| `s2-qa-run.sh` | 10897 | `af614b705605006f78807c242d4058f06fc4af3cb481907b1bb5e1a1bda7a0e5` | byte-identical to as8p |
| `s2-qa-selftest.cjs` | 10552 | `d1bf5f4e2a91135d7cd05bfa024ce32ffc615c0428add293f788eae01e60a35c` | byte-identical to as8p |
| `s2-qa-stepped.cjs` | 5723 | `89bfe4728ff705df8b879705e809c09df19f669e1728b3b380b8e4836f12b12e` | byte-identical to as8p |
| `scroll-evidence.cjs` | 1500 | `548564e5220d101974705e934bd70a6ff6972c7ac37e1a3a26337e50e54e469b` | byte-identical to as8p |

## Changed files (extract verbatim over the as8p package; run with `bash`)

~~~ s2-qa-matrix.json
{
 "head": "cf309782e94afd38b2acbe8a3c2c1a96fc005220",
 "ciPipeline": 3013528423,
 "htmlJob": 17639326311,
 "zipSha256": "4e11043dc1da239a3a741e930de198ec4571bd910900008d4aa64b6294e3d3a0",
 "agfCssBlob": "31864b3f27a7a2703cd6da06467797689ae0e7f7",
 "prefix": "/agents-govern-framework/",
 "stepPx": 320,
 "maxGestures": 3,
 "maxTabPresses": 150,
 "caps": {
  "memoryMax": "512M",
  "swapMax": 0,
  "runtimeSec": 90,
  "nodeOldSpaceMiB": 48,
  "hostMemAvailableFloorMiB": 1024
 },
 "a3Helpers": {
  "code-measure.cjs": "3546a0396305d6b4e6085393de7f1aa4bd03887192f545866491560bd0a1a15e",
  "code-proof.cjs": "2859dd21cac883249e9b2dd98f73e2dc80fc388ebad5168d03d1dc7949a846df",
  "scroll-evidence.cjs": "548564e5220d101974705e934bd70a6ff6972c7ac37e1a3a26337e50e54e469b"
 },
 "modes": [
  {
   "id": "desktop-light",
   "width": 1440,
   "height": 900,
   "colorScheme": "light",
   "scheme": "default"
  },
  {
   "id": "desktop-dark",
   "width": 1440,
   "height": 900,
   "colorScheme": "dark",
   "scheme": "slate"
  },
  {
   "id": "mobile-light",
   "width": 390,
   "height": 844,
   "colorScheme": "light",
   "scheme": "default"
  },
  {
   "id": "mobile-dark",
   "width": 390,
   "height": 844,
   "colorScheme": "dark",
   "scheme": "slate"
  }
 ],
 "cases": [
  {
   "id": "formula",
   "page": "docs/adopt/tooling-and-tokens/",
   "selector": "article > p:nth-of-type(27) + div.highlight > pre > code",
   "blockSelector": "article > p:nth-of-type(27) + div.highlight",
   "codeText": "estimated_tokens_saved =\n  tokens(manual_context_that_would_have_been_read)\n  - tokens(tool_output_that_was_read)\n  - tokens(extra_prompt_needed_to_invoke_the_tool)\n",
   "codeTextSha256": "5bc6bd8deeaa98eec3285cafdf689126c84c145cd193095ec7b6d2388f03dd24",
   "nativeWheelEligible": true,
   "images": [],
   "rows": 4,
   "maxRow": 50
  },
  {
   "id": "exports",
   "page": "docs/adopt/tooling-and-tokens/",
   "selector": "article > p:nth-of-type(31) + div.highlight > pre > code",
   "blockSelector": "article > p:nth-of-type(31) + div.highlight",
   "codeText": "export AGOV_TOKEN_ROI_LOG=\"$PWD/tmp/codex-agov-token-roi.jsonl\"\n# or, for a Claude-run shell:\nexport AGOV_TOKEN_ROI_LOG=\"$PWD/tmp/claude-agov-token-roi.jsonl\"\n",
   "codeTextSha256": "8b632bc1d768a595b792395063d8fb858269f4e69be93d2a9f9fcd5a487709e5",
   "nativeWheelEligible": true,
   "images": [
    "desktop-light",
    "desktop-dark",
    "mobile-light",
    "mobile-dark"
   ],
   "rows": 3,
   "maxRow": 64
  },
  {
   "id": "report-command",
   "page": "docs/adopt/tooling-and-tokens/",
   "selector": "article > p:nth-of-type(33) + div.highlight > pre > code",
   "blockSelector": "article > p:nth-of-type(33) + div.highlight",
   "codeText": "agov token roi report --log tmp/codex-agov-token-roi.jsonl\n",
   "codeTextSha256": "a5b44b06bf5f4d23f9cbee59f8460bccad260c38a59ca46befb249b9f234b549",
   "nativeWheelEligible": true,
   "images": [],
   "rows": 1,
   "maxRow": 58
  },
  {
   "id": "long-row",
   "page": "reference/review-findings/",
   "selector": "article > p:nth-of-type(14) + div.highlight > pre > code",
   "blockSelector": "article > p:nth-of-type(14) + div.highlight",
   "codeText": "Finding: <identifier>\nImpact: critical | material | minor | unknown\nEnforcement: blocking | advisory\nBasis: <named rule/requirement, or explicitly advisory judgment>\nFinding state: open | resolved | superseded\nRisk disposition: none | accepted\nRisk decision: <required when accepted: decision ID, scope/revision, revisit condition>\nResolution condition: <observable condition>\n",
   "codeTextSha256": "b5f413effbbd6bfa5293dd788db7e6aea596806f2e907d8dab086d7c5e2edb38",
   "nativeWheelEligible": true,
   "images": [
    "desktop-light",
    "desktop-dark",
    "mobile-light",
    "mobile-dark"
   ],
   "rows": 8,
   "maxRow": 87
  },
  {
   "id": "synthetic-nested",
   "page": "docs/adopt/synthetic-nested/",
   "selector": "article .admonition > div.highlight > pre > code",
   "blockSelector": "article .admonition > div.highlight",
   "codeText": "agov token roi report --log tmp/codex-agov-token-roi.jsonl\n",
   "codeTextSha256": "a5b44b06bf5f4d23f9cbee59f8460bccad260c38a59ca46befb249b9f234b549",
   "nativeWheelEligible": true,
   "images": [],
   "rows": 1,
   "maxRow": 58,
   "synthetic": true,
   "derivedFrom": "report-command"
  }
 ],
 "witnessProjection": {
  "file": "saved-code-views-projection.json",
  "sha256": "cdd4b1c84867bfef56dd61ede61231aa221aaf3d845efa9662693426e499b382",
  "bytes": 172093
 },
 "s1Serve": "40db4af6b3e5c634ba1d121ca7daad018172805fb23ddd529ecbca9c1ca6ec06"
}
~~~

~~~ s2-qa-hashes.sha256
3546a0396305d6b4e6085393de7f1aa4bd03887192f545866491560bd0a1a15e  code-measure.cjs
2859dd21cac883249e9b2dd98f73e2dc80fc388ebad5168d03d1dc7949a846df  code-proof.cjs
f22ff058196d788c635c5ace6b2d64e3b0bef3f4bf584307ca10198122d717ab  real-cases.json
40db4af6b3e5c634ba1d121ca7daad018172805fb23ddd529ecbca9c1ca6ec06  s1-qa-serve.cjs
d3bc469540e27b3fbb608321029a89c68819941f2a619995dfb8d9d6344502bf  s2-make-synthetic.cjs
361038a3d17bc1583c83d699918aaa888aef1b900215a091a894eb55512d913b  s2-qa-matrix.json
1b01ea7d8f6681e834dc77d371f5d2eb0ebe2f65db2f49be2489e19b00bfc56c  s2-qa-observe.cjs
9bcc9d9c9e39c23a7f0d592ad6ee09b302ea9df75803dbb8b5c6557acd611cb5  s2-qa-run-selftest.sh
af614b705605006f78807c242d4058f06fc4af3cb481907b1bb5e1a1bda7a0e5  s2-qa-run.sh
d1bf5f4e2a91135d7cd05bfa024ce32ffc615c0428add293f788eae01e60a35c  s2-qa-selftest.cjs
89bfe4728ff705df8b879705e809c09df19f669e1728b3b380b8e4836f12b12e  s2-qa-stepped.cjs
548564e5220d101974705e934bd70a6ff6972c7ac37e1a3a26337e50e54e469b  scroll-evidence.cjs
~~~

Nothing here authorizes a run, a stage, Gate 2 for S2, or a release.
