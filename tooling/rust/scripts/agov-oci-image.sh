#!/usr/bin/env sh
set -eu

usage() {
  cat <<'EOF'
Usage:
  agov-oci-image.sh check-tag
  agov-oci-image.sh build
  agov-oci-image.sh build-unpublished
  agov-oci-image.sh scan
  agov-oci-image.sh scan-unpublished
  agov-oci-image.sh push
  agov-oci-image.sh provenance
  agov-oci-image.sh image-ref
  agov-oci-image.sh unpublished-ref
  agov-oci-image.sh verify-ref <reference>
  agov-oci-image.sh selftest

Environment:
  AGOV_VERSION        Required. Version from agf/VERSION.
  CI_COMMIT_TAG       Required for check-tag/build/push. Must be v$AGOV_VERSION.
  CI_COMMIT_SHA       Required for build/provenance labels.
  CI_REGISTRY_IMAGE   Required for build/push/provenance. Base registry image path.
  CI_REGISTRY_USER    Required for push.
  CI_REGISTRY_PASSWORD Required for push.

`provenance` builds an artifact_kind: oci-image release-provenance record
(agf/schemas/release-provenance.schema.json) for the image `push` just
published, using the digest `push` wrote to
dist/agov-oci-digest-$AGOV_VERSION.txt. Run push before provenance. The
agov binary used to build the record is extracted from the already-built
dist/agov-$AGOV_VERSION-linux-x86_64.tar.gz tooling archive
(a static musl binary, runnable in this image) -- it is never rebuilt here.
Archive filenames use the public platform slug (linux-x86_64 etc.);
AGOV_TARGET still carries the real Rust triple for the staged binary.
EOF
}

SCRIPT_DIR=$(CDPATH= cd "$(dirname "$0")" && pwd)
TARGETS_MANIFEST="$SCRIPT_DIR/../release-targets.txt"

# Public archive filename slug for a Rust target triple. Only the archive
# *filename* uses the slug (Workstream A naming decision, er5h); AGOV_TARGET
# itself keeps the real triple for build/binary selection.
#
# Deliberately maps ONLY the Linux tar.gz targets this OCI path can consume:
# both callers construct a .tar.gz path, extract with tar, and run an
# extensionless `agov` ELF binary inside this Linux image. The Windows
# release artifact (agov-...-windows-x86_64.zip, containing agov.exe) can
# never satisfy that, so x86_64-pc-windows-gnu is rejected as unsupported
# here rather than translated into a nonexistent .tar.gz path.
manifest_records() {
  manifest="$1"
  awk '
    function reject(message) {
      printf "invalid release-target manifest %s:%d: %s\n", FILENAME, NR, message > "/dev/stderr"
      failed = 1
    }
    function unsafe(value) {
      return value ~ /[\\\/]/ || index(value, "..") || value ~ /^-/ || value ~ /[[:cntrl:]]/
    }
    /^[[:space:]]*$/ { next }
    /^#/ { next }
    {
      if (NF != 2) {
        reject("expected exactly two fields: <rust-triple> <public-slug>")
        next
      }
      if (unsafe($1) || unsafe($2)) {
        reject("fields must be safe path-component tokens (no slash, backslash, .., leading -, whitespace, or control characters)")
        next
      }
      if (targets[$1]++) {
        reject("duplicate target: " $1)
        next
      }
      if (slugs[$2]++) {
        reject("duplicate slug: " $2)
        next
      }
      count++
      target[count] = $1
      slug[count] = $2
    }
    END {
      if (failed) exit 1
      if (count == 0) {
        printf "invalid release-target manifest %s: no target records\n", FILENAME > "/dev/stderr"
        exit 1
      }
      for (i = 1; i <= count; i++) print target[i], slug[i]
    }
  ' "$manifest"
}

# Private injection seam for selftest only. Production callers cannot choose
# the manifest path: target_slug() below always supplies TARGETS_MANIFEST.
target_slug_from() {
  manifest="$1"
  requested="$2"
  if ! records=$(manifest_records "$manifest"); then
    return 1
  fi
  found=""
  while IFS=' ' read -r target slug; do
    if [ "$target" = "$requested" ]; then
      found="$slug"
      break
    fi
  done <<EOF
$records
EOF
  if [ -z "$found" ]; then
    echo "no public filename slug mapped for target $requested" >&2
    return 1
  fi
  printf '%s' "$found"
}

target_slug() {
  requested="$1"
  if [ "$requested" = "x86_64-pc-windows-gnu" ]; then
    echo "target $requested is not supported by this Linux OCI-image path (its release artifact is a Windows .zip with agov.exe, not a .tar.gz with a runnable agov binary)" >&2
    return 1
  fi
  target_slug_from "$TARGETS_MANIFEST" "$requested"
}

selftest() {
  tmp=$(mktemp -d)
  trap 'rm -rf "$tmp"' EXIT INT TERM
  failed=0

  records=$(manifest_records "$TARGETS_MANIFEST")
  while IFS=' ' read -r target expected_slug; do
    [ -n "$target" ] || continue
    actual_slug=$(target_slug "$target")
    if [ "$actual_slug" != "$expected_slug" ]; then
      echo "target_slug mismatch for $target: expected $expected_slug, got $actual_slug" >&2
      failed=1
    fi
  done <<EOF
$records
EOF

  windows_output=$(target_slug x86_64-pc-windows-gnu 2>&1 || true)
  if target_slug x86_64-pc-windows-gnu >/dev/null 2>&1 ||
     ! printf '%s\n' "$windows_output" | grep -qF 'not supported by this Linux OCI-image path'; then
    echo "Windows target did not produce its specific rejection" >&2
    failed=1
  fi

  unknown_output=$(target_slug unknown-target 2>&1 || true)
  if target_slug unknown-target >/dev/null 2>&1 ||
     ! printf '%s\n' "$unknown_output" | grep -qF 'no public filename slug mapped'; then
    echo "unknown target did not fail clearly" >&2
    failed=1
  fi

  malformed="$tmp/malformed.txt"
  printf '%s\n' 'target-without-slug' > "$malformed"
  malformed_output=$(target_slug_from "$malformed" target-without-slug 2>&1 || true)
  if target_slug_from "$malformed" target-without-slug >/dev/null 2>&1 ||
     ! printf '%s\n' "$malformed_output" | grep -qF 'expected exactly two fields'; then
    echo "malformed manifest did not fail clearly" >&2
    failed=1
  fi

  # A2 negative proofs. These assert that a wrong OCI reference FAILS, so
  # the gate is demonstrated to fire without the canonical acceptance
  # pipeline going red (gate-self-test-rule.md; h3nb section 1.5).
  ref_env='CI_REGISTRY_IMAGE=registry.example.com/group/project AGOV_VERSION=9.9.9'
  canonical='registry.example.com/group/project/agov:9.9.9'

  ok_output=$(env $ref_env "$0" verify-ref "$canonical" 2>&1 || true)
  if ! env $ref_env "$0" verify-ref "$canonical" >/dev/null 2>&1; then
    echo "verify-ref rejected the canonical reference: $ok_output" >&2
    failed=1
  fi

  # The exact v2.8.0 defect: a leading `v` on the tag component.
  bad_v='registry.example.com/group/project/agov:v9.9.9'
  bad_v_output=$(env $ref_env "$0" verify-ref "$bad_v" 2>&1 || true)
  if env $ref_env "$0" verify-ref "$bad_v" >/dev/null 2>&1 ||
     ! printf '%s\n' "$bad_v_output" | grep -qF 'OCI reference mismatch'; then
    echo "v-prefixed reference was not rejected (the v2.8.0 defect)" >&2
    failed=1
  fi

  # A different repository component must also be rejected.
  bad_repo='registry.example.com/group/project/agov-cli:9.9.9'
  bad_repo_output=$(env $ref_env "$0" verify-ref "$bad_repo" 2>&1 || true)
  if env $ref_env "$0" verify-ref "$bad_repo" >/dev/null 2>&1 ||
     ! printf '%s\n' "$bad_repo_output" | grep -qF 'OCI reference mismatch'; then
    echo "mismatched repository component was not rejected" >&2
    failed=1
  fi

  # A missing environment must fail loudly rather than emit `/agov:`.
  empty_output=$(env -u CI_REGISTRY_IMAGE AGOV_VERSION=9.9.9 "$0" image-ref 2>&1 || true)
  if env -u CI_REGISTRY_IMAGE AGOV_VERSION=9.9.9 "$0" image-ref >/dev/null 2>&1 ||
     ! printf '%s\n' "$empty_output" | grep -qF 'missing required environment variable: CI_REGISTRY_IMAGE'; then
    echo "image-ref did not fail on missing CI_REGISTRY_IMAGE" >&2
    failed=1
  fi

  # image-ref and verify-ref must agree, or the single-source claim is false.
  emitted=$(env $ref_env "$0" image-ref)
  if [ "$emitted" != "$canonical" ]; then
    echo "image-ref emitted '$emitted', expected '$canonical'" >&2
    failed=1
  fi

  # A1 structural proofs. The MR-stage path must be usable without a tag,
  # must not require a registry namespace, and must not be able to name one;
  # the published path's tag guard must remain intact. All assert on error
  # text, so none of them needs podman or a built archive.
  mr_env='AGOV_VERSION=9.9.9 CI_COMMIT_SHA=deadbeefdeadbeefdeadbeefdeadbeefdeadbeef'

  # build-unpublished must get PAST the tag gate: it fails on the absent
  # archive, not on a missing CI_COMMIT_TAG.
  unpub_output=$(env -u CI_COMMIT_TAG -u CI_REGISTRY_IMAGE $mr_env "$0" build-unpublished 2>&1 || true)
  if ! printf '%s\n' "$unpub_output" | grep -qF 'missing release archive'; then
    echo "build-unpublished did not reach the archive stage: $unpub_output" >&2
    failed=1
  fi
  if printf '%s\n' "$unpub_output" | grep -qE 'CI_COMMIT_TAG|CI_REGISTRY_IMAGE'; then
    echo "build-unpublished still demands tag or registry environment" >&2
    failed=1
  fi

  # The published path must still refuse to build without a tag.
  pub_output=$(env -u CI_COMMIT_TAG $ref_env CI_COMMIT_SHA=deadbeef "$0" build 2>&1 || true)
  if env -u CI_COMMIT_TAG $ref_env CI_COMMIT_SHA=deadbeef "$0" build >/dev/null 2>&1 ||
     ! printf '%s\n' "$pub_output" | grep -qF 'missing required environment variable: CI_COMMIT_TAG'; then
    echo "published build path lost its tag guard" >&2
    failed=1
  fi

  # The unpublished reference must be local-only and must never collide with
  # a registry namespace (h3nb section 1.4, enforced structurally).
  unpub_ref=$(env -u CI_REGISTRY_IMAGE AGOV_VERSION=9.9.9 "$0" unpublished-ref)
  case "$unpub_ref" in
    localhost/*) ;;
    *) echo "unpublished reference is not localhost-scoped: $unpub_ref" >&2; failed=1 ;;
  esac
  if [ "$unpub_ref" = "$canonical" ]; then
    echo "unpublished reference collides with the published reference" >&2
    failed=1
  fi

  if [ "$failed" -ne 0 ]; then
    echo "selftest FAILED" >&2
    return 1
  fi
  echo "selftest OK: canonical targets resolve; Windows, unknown, and malformed inputs are rejected; OCI reference is single-sourced and mismatched spellings are rejected"
}

require_env() {
  name=$1
  eval "value=\${$name:-}"
  if [ -z "$value" ]; then
    echo "missing required environment variable: $name" >&2
    exit 1
  fi
}

check_tag() {
  require_env AGOV_VERSION
  require_env CI_COMMIT_TAG
  tag_version=${CI_COMMIT_TAG#v}
  if [ "$tag_version" != "$AGOV_VERSION" ]; then
    echo "Tag $CI_COMMIT_TAG does not match agf/VERSION $AGOV_VERSION" >&2
    exit 1
  fi
}

# The single source of truth for the published OCI reference. Every path
# that builds, scans, pushes, or *checks* the image must derive its name
# from here rather than composing one independently.
#
# v2.8.0 shipped with exactly that divergence: the immediately-pre-tag
# namespace absence check queried `v2.8.0` while this function publishes
# `agov:$AGOV_VERSION` (no `v`). The correct namespace was therefore never
# checked for absence, and because OCI tags are mutable a successful push
# is not retrospective proof it was free. Disclosed in `q5mm`, accepted as
# a process-evidence gap in `gs18`, and closed here by making the checked
# name and the published name the same derivation.
# Deliberately pure: no require_env here. Every caller invokes this inside
# a command substitution, where `exit` from require_env would terminate only
# the subshell and yield an empty reference to the parent -- silently
# producing `/agov:` instead of failing. Callers validate the environment in
# their own scope before substituting; see print_image_ref and verify_ref.
image_ref() {
  printf '%s/agov:%s' "$CI_REGISTRY_IMAGE" "$AGOV_VERSION"
}

# Validate the environment in the caller's own scope, then emit the single
# published reference. This is what the release runbook's pre-tag namespace
# check consumes, so the name it queries for absence is by construction the
# name publication will use.
print_image_ref() {
  require_env CI_REGISTRY_IMAGE
  require_env AGOV_VERSION
  image_ref
  printf '\n'
}

# The unpublished counterpart, exposed so the selftest asserts against the
# real derivation rather than a restatement of it.
print_unpublished_ref() {
  require_env AGOV_VERSION
  local_image_ref
  printf '\n'
}

# Operator-facing guard for the pre-tag namespace check. The runbook passes
# whatever reference it is about to query; anything not byte-identical to
# image_ref() is rejected.
#
# v2.8.0 shipped with exactly that divergence: the pre-tag absence check
# queried `v2.8.0` while publication used `agov:$AGOV_VERSION` (no `v`), so
# the correct namespace was never checked for absence. Because OCI tags are
# mutable, a successful push is not retrospective proof the name was free.
# Disclosed in `q5mm`, accepted as a process-evidence gap in `gs18`.
verify_ref() {
  require_env CI_REGISTRY_IMAGE
  require_env AGOV_VERSION
  candidate=${1:-}
  if [ -z "$candidate" ]; then
    echo "verify-ref requires the candidate reference as its argument" >&2
    return 2
  fi
  expected=$(image_ref)
  if [ "$candidate" != "$expected" ]; then
    printf 'OCI reference mismatch: checked %s but publication uses %s\n' \
      "$candidate" "$expected" >&2
    return 1
  fi
  printf 'OCI reference OK: %s\n' "$expected"
}

podman_bin() {
  printf '%s' "${PODMAN:-podman}"
}

# Local-only reference for the unpublished MR-stage path. The `localhost/`
# prefix is not a registry namespace, so no MR-stage code path can name,
# reach, or overwrite a release namespace even if invoked wrongly. This is
# h3nb section 1.4 enforced structurally rather than by convention.
local_image_ref() {
  printf 'localhost/agov-unpublished:%s' "$AGOV_VERSION"
}

# Shared build body. Takes the target reference so the published and
# unpublished paths cannot drift apart, for the same single-source reason
# A2 exists.
build_with_ref() {
  build_ref=$1
  require_env CI_COMMIT_SHA

  target=${AGOV_TARGET:-x86_64-unknown-linux-musl}
  slug=$(target_slug "$target")
  archive="dist/agov-${AGOV_VERSION}-${slug}.tar.gz"
  if [ ! -f "$archive" ]; then
    echo "missing release archive: $archive" >&2
    exit 1
  fi

  staging=$(mktemp -d)
  trap 'rm -rf "$staging"' EXIT INT TERM
  tar -xzf "$archive" -C "$staging"
  cp "$staging/agov-${AGOV_VERSION}-${slug}/agov" "$staging/agov"

  "$(podman_bin)" build \
    --file tooling/rust/Containerfile.agov \
    --build-arg "AGOV_VERSION=$AGOV_VERSION" \
    --build-arg "AGOV_REVISION=$CI_COMMIT_SHA" \
    --tag "$build_ref" \
    "$staging"
}

# Published path: tag-gated, unchanged in behavior.
build_image() {
  check_tag
  require_env CI_REGISTRY_IMAGE
  require_env AGOV_VERSION
  build_with_ref "$(image_ref)"
}

# Unpublished MR-stage path (A1). No check_tag: CI_COMMIT_TAG does not exist
# in a merge-request pipeline, which is exactly why the release path was not
# reusable here -- the advisory finding in review 58kk. CI_REGISTRY_IMAGE is
# deliberately not required: this path must not need, and must not be able to
# use, a registry namespace.
build_unpublished() {
  require_env AGOV_VERSION
  build_with_ref "$(local_image_ref)"
}

# Shared scan body, parameterised by reference for the same reason as
# build_with_ref: the MR-stage gate and the tag-stage gate must be the
# identical Trivy invocation, or the MR gate proves nothing about the
# release. Same image, same severities, same --ignore-unfixed, same
# digest-pinned scanner, same --exit-code 1.
scan_with_ref() {
  scan_ref=$1
  image_archive=$(mktemp)
  trap 'rm -f "$image_archive"' EXIT INT TERM
  # docker-archive, not oci-archive: Trivy 0.66's --input path cannot open
  # an oci-archive tar as either a Docker image or an OCI directory.
  "$(podman_bin)" save --format docker-archive --output "$image_archive" "$scan_ref"
  # Fully qualified + digest-pinned: an unqualified "aquasec/trivy:0.66.0"
  # is rejected by Podman in this environment, and a tag alone is mutable.
  # No trailing image-ref argument: with --input, Trivy scans the archive
  # directly; a second positional image name is redundant and was the
  # third bug in the original invocation.
  "$(podman_bin)" run --rm \
    -v "$image_archive:/image.tar:ro" \
    docker.io/aquasec/trivy@sha256:086971aaf400beebd94e8300fd8ea623774419597169156cec56eec5b00dfb1e \
    image \
    --input /image.tar \
    --severity HIGH,CRITICAL \
    --ignore-unfixed \
    --exit-code 1
}

scan_image() {
  require_env CI_REGISTRY_IMAGE
  require_env AGOV_VERSION
  scan_with_ref "$(image_ref)"
}

scan_unpublished() {
  require_env AGOV_VERSION
  scan_with_ref "$(local_image_ref)"
}

digest_file() {
  printf 'dist/agov-oci-digest-%s.txt' "$AGOV_VERSION"
}

push_image() {
  check_tag
  require_env CI_REGISTRY_IMAGE
  require_env CI_REGISTRY
  require_env CI_REGISTRY_USER
  require_env CI_REGISTRY_PASSWORD

  printf '%s' "$CI_REGISTRY_PASSWORD" |
    "$(podman_bin)" login "$CI_REGISTRY" --username "$CI_REGISTRY_USER" --password-stdin
  mkdir -p dist
  "$(podman_bin)" push --digestfile "$(digest_file)" "$(image_ref)"
}

provenance_record() {
  require_env AGOV_VERSION
  require_env CI_COMMIT_SHA
  require_env CI_REGISTRY_IMAGE

  digestfile="$(digest_file)"
  if [ ! -f "$digestfile" ]; then
    echo "missing digest file: $digestfile (run push first)" >&2
    exit 1
  fi
  digest="$(cat "$digestfile")"

  target=${AGOV_TARGET:-x86_64-unknown-linux-musl}
  slug=$(target_slug "$target")
  archive="dist/agov-${AGOV_VERSION}-${slug}.tar.gz"
  if [ ! -f "$archive" ]; then
    echo "missing release archive: $archive" >&2
    exit 1
  fi

  staging=$(mktemp -d)
  trap 'rm -rf "$staging"' EXIT INT TERM
  tar -xzf "$archive" -C "$staging"
  agov_bin="$staging/agov-${AGOV_VERSION}-${slug}/agov"

  "$agov_bin" dist provenance-record oci \
    --product agov \
    --version "$AGOV_VERSION" \
    --source-commit "$CI_COMMIT_SHA" \
    --tag "${CI_COMMIT_TAG:-untagged}" \
    --build-timestamp "$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
    --pipeline-id "${CI_PIPELINE_ID:-unknown}" \
    --job-id "${CI_JOB_ID:-unknown}" \
    --implementation rust \
    --packaging oci-image \
    --image-reference "$(image_ref)" \
    --manifest-digest "$digest" \
    --label-version "$AGOV_VERSION" \
    --label-revision "$CI_COMMIT_SHA" \
    --output "dist/agov-oci-${AGOV_VERSION}-provenance.json"
}

case "${1:-}" in
  check-tag)
    check_tag
    ;;
  build)
    build_image
    ;;
  build-unpublished)
    build_unpublished
    ;;
  scan)
    scan_image
    ;;
  scan-unpublished)
    scan_unpublished
    ;;
  push)
    push_image
    ;;
  provenance)
    provenance_record
    ;;
  image-ref)
    print_image_ref
    ;;
  unpublished-ref)
    print_unpublished_ref
    ;;
  verify-ref)
    shift
    verify_ref "${1:-}"
    ;;
  selftest)
    selftest
    ;;
  -h|--help|help)
    usage
    ;;
  *)
    usage >&2
    exit 2
    ;;
esac
