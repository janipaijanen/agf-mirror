#!/usr/bin/env sh
set -eu

usage() {
  cat <<'EOF'
Usage:
  agov-oci-image.sh check-tag
  agov-oci-image.sh build
  agov-oci-image.sh scan
  agov-oci-image.sh push
  agov-oci-image.sh provenance

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
dist/agov-$AGOV_VERSION-x86_64-unknown-linux-musl.tar.gz tooling archive
(a static musl binary, runnable in this image) -- it is never rebuilt here.
EOF
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

image_ref() {
  printf '%s/agov:%s' "$CI_REGISTRY_IMAGE" "$AGOV_VERSION"
}

podman_bin() {
  printf '%s' "${PODMAN:-podman}"
}

build_image() {
  check_tag
  require_env CI_COMMIT_SHA
  require_env CI_REGISTRY_IMAGE

  target=${AGOV_TARGET:-x86_64-unknown-linux-musl}
  archive="dist/agov-${AGOV_VERSION}-${target}.tar.gz"
  if [ ! -f "$archive" ]; then
    echo "missing release archive: $archive" >&2
    exit 1
  fi

  staging=$(mktemp -d)
  trap 'rm -rf "$staging"' EXIT INT TERM
  tar -xzf "$archive" -C "$staging"
  cp "$staging/agov-${AGOV_VERSION}-${target}/agov" "$staging/agov"

  "$(podman_bin)" build \
    --file tooling/rust/Containerfile.agov \
    --build-arg "AGOV_VERSION=$AGOV_VERSION" \
    --build-arg "AGOV_REVISION=$CI_COMMIT_SHA" \
    --tag "$(image_ref)" \
    "$staging"
}

scan_image() {
  require_env CI_REGISTRY_IMAGE
  require_env AGOV_VERSION
  image_archive=$(mktemp)
  trap 'rm -f "$image_archive"' EXIT INT TERM
  # docker-archive, not oci-archive: Trivy 0.66's --input path cannot open
  # an oci-archive tar as either a Docker image or an OCI directory.
  "$(podman_bin)" save --format docker-archive --output "$image_archive" "$(image_ref)"
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
  archive="dist/agov-${AGOV_VERSION}-${target}.tar.gz"
  if [ ! -f "$archive" ]; then
    echo "missing release archive: $archive" >&2
    exit 1
  fi

  staging=$(mktemp -d)
  trap 'rm -rf "$staging"' EXIT INT TERM
  tar -xzf "$archive" -C "$staging"
  agov_bin="$staging/agov-${AGOV_VERSION}-${target}/agov"

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
  scan)
    scan_image
    ;;
  push)
    push_image
    ;;
  provenance)
    provenance_record
    ;;
  -h|--help|help)
    usage
    ;;
  *)
    usage >&2
    exit 2
    ;;
esac
