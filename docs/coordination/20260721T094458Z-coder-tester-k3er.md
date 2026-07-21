---
kind: review
from: coder-tester
to: coder-tester
id: 20260721T094458Z-coder-tester-k3er
timestamp: 2026-07-21T09:44:58Z
immutable: true
references-to:
  - 20260721T093448Z-coder-tester-7mhk
  - 20260721T093001Z-coder-tester-skm5
topic: v2-3-0-rust-checksum-consumer-path-block
requires-ack: true
artifacts_owned: []
---

# BLOCK: Rust release checksums fail in consumer layout

Independent review downloaded the preview artifacts produced by MR !306 jobs
`15448242126` and `15448242127`. The framework archive checksum files verify
normally beside their archives. Both Rust checksum files instead name their
archive as `../../dist/agov-2.3.0-<target>.tar.gz`.

Consequently, the standard release-consumer operation below fails when the
checksum and archive are downloaded into the same directory:

```text
sha256sum -c agov-2.3.0-x86_64-unknown-linux-musl.tar.gz.sha256
sha256sum: ../../dist/agov-2.3.0-x86_64-unknown-linux-musl.tar.gz: No such file or directory
```

Direct recomputation shows that the recorded digest bytes are correct. The
defect is the path embedded in the published checksum interface.

Before merge, fix checksum generation to record the archive basename, add or
run a consumer-layout check against the produced artifacts, and obtain a green
content pipeline containing regenerated preview artifacts. Supersede the final
Gate 2 handoff with the corrected content head and evidence.

Also supersede the minor factual error in `7mhk`: that immutable record calls
`98c3b6f` the branch tip although the record itself was added by the subsequent
tip `bc57265`.
