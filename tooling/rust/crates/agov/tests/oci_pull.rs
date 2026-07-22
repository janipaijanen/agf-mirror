//! Composed real-registry evidence: pulls a disposable public image
//! (`docker.io/library/busybox`, not this project's own registry) and
//! proves `validate-provenance --oci-image` rejects a record whose
//! `labels` claim OCI version/revision labels the real image does not
//! carry, using real `podman pull`/`podman inspect` output rather than a
//! synthetic fixture. Requires network access and a working `podman`.
//!
//! This is the "approved disposable/public registry pull test" from
//! `hmpr`'s !351 Gate 2 acceptance matrix (row B): proof that the
//! pull-by-digest -> inspect -> compare mechanics work end-to-end against
//! a real registry, decoupled from this project's own OCI credentials.
//!
//! `#[ignore]`d deliberately: a missing `podman` or missing network is an
//! *infrastructure* failure for this test, not a passing test (per
//! `kaka`'s BLOCK B finding against an earlier version of this file that
//! silently returned success when podman/network were unavailable). The
//! generic `rust:tooling:test` CI job (plain `rust:1`, no podman) must
//! not attempt this test at all; the dedicated `rust:tooling:oci-pull-test`
//! CI job (`quay.io/podman/stable`, real network) runs it with `--ignored`
//! where podman and network are guaranteed, so failure here always means
//! a real defect, never a missing dependency.

use std::process::Command;

const BUSYBOX_DIGEST: &str =
    "sha256:fd8d9aa63ba2f0982b5304e1ee8d3b90a210bc1ffb5314d980eb6962f1a9715d";
const BUSYBOX_REF: &str = "docker.io/library/busybox";

fn agov(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_agov"))
        .env("AGOV_TOKEN_ROI", "0")
        .args(args)
        .output()
        .expect("run agov")
}

#[test]
#[ignore = "requires a real podman binary and real network access; run explicitly via \
            `cargo test -p agov --test oci_pull -- --ignored`, as the dedicated \
            rust:tooling:oci-pull-test CI job does"]
fn real_disposable_registry_pull_rejects_record_with_wrong_labels() {
    let pinned = format!("{BUSYBOX_REF}@{BUSYBOX_DIGEST}");
    let pull_status = Command::new("podman")
        .args(["pull", &pinned])
        .status()
        .expect(
            "podman must be installed and runnable in this test's environment -- a missing \
             podman is an infrastructure failure for this ignored test, not a reason to skip it",
        );
    assert!(
        pull_status.success(),
        "podman pull {pinned} must succeed -- a failed pull (e.g. no network) is an \
         infrastructure failure for this ignored test, not a reason to skip it"
    );

    let dir = std::env::temp_dir().join(format!("agov-oci-pull-test-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let record_path = dir.join("busybox-provenance.json");
    let record = serde_json::json!({
        "schema_version": "2.4",
        "product": "agov",
        "artifact_kind": "oci-image",
        "version": "2.4.0",
        "source_commit": "abc123def456",
        "tag": "v2.4.0",
        "build_timestamp": "2026-07-22T00:00:00Z",
        "implementation": "rust",
        "packaging": "oci-image",
        "image_reference": BUSYBOX_REF,
        "manifest_digest": BUSYBOX_DIGEST,
        "labels": {
            "version": "2.4.0",
            "revision": "abc123def456"
        }
    });
    std::fs::write(&record_path, serde_json::to_string_pretty(&record).unwrap()).unwrap();

    let output = agov(&[
        "dist",
        "validate-provenance",
        record_path.to_str().unwrap(),
        "--oci-image",
        &pinned,
    ]);

    let _ = std::fs::remove_dir_all(&dir);

    // busybox carries no org.opencontainers.image.version/revision labels
    // matching our fabricated record, so this must fail -- proving the
    // real pull+inspect+compare pipeline actually inspects real evidence
    // rather than trivially passing.
    assert_ne!(
        output.status.code(),
        Some(0),
        "expected rejection against real busybox image evidence; stdout={} stderr={}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("OCI image evidence disagreement"),
        "expected a real evidence disagreement finding, got: {stderr}"
    );
}
