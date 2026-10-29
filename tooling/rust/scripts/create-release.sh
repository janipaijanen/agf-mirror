#!/bin/sh
# SPDX-License-Identifier: MIT
# Copyright (c) 2026 Jani Päijänen
#
# Create-only GitLab Release publisher for the tag pipeline. The caller runs
# `preflight` before package uploads and `publish` immediately afterward with
# the same payload file. No command updates, deletes, or creates a Git tag.

set -eu

SCRIPT_DIR=$(CDPATH= cd "$(dirname "$0")" && pwd)

die() {
  printf 'release:create failed: %s\n' "$1" >&2
  exit 1
}

require_command() {
  command -v "$1" >/dev/null 2>&1 || die "required command unavailable: $1"
}

require_context() {
  local name value
  for name in CI_API_V4_URL CI_PROJECT_ID CI_PROJECT_PATH CI_COMMIT_TAG CI_COMMIT_SHA CI_JOB_TOKEN; do
    eval "value=\${$name-}"
    [ -n "$value" ] || die "required CI context is empty: $name"
  done
  case "$CI_API_V4_URL" in
    https://*) : ;;
    *) die "CI_API_V4_URL must use https" ;;
  esac
  case "$CI_PROJECT_ID" in
    *[!0-9]*|'') die "CI_PROJECT_ID must be numeric" ;;
  esac
  case "$CI_COMMIT_SHA" in
    *[!0-9a-fA-F]*|'') die "CI_COMMIT_SHA must be a hexadecimal commit id" ;;
  esac
}

encoded_tag() {
  printf '%s' "$CI_COMMIT_TAG" | jq -sRr '@uri'
}

package_base() {
  require_command jq
  require_context
  printf '%s/projects/%s/packages/generic/agf/%s\n' \
    "${CI_API_V4_URL%/}" "$CI_PROJECT_ID" "$(encoded_tag)"
}

# Run curl with the credential supplied only through stdin as curl config.
# `--disable` is deliberately the first argument so ambient curl config is
# ignored. Do not add --location or --retry to this transport.
http_request() {
  local method url body_file payload_file
  method="$1"
  url="$2"
  body_file="$3"
  payload_file="${4-}"

  if [ -n "$payload_file" ]; then
    printf 'header = "JOB-TOKEN: %s"\n' "$CI_JOB_TOKEN" |
      curl --disable --config - \
        --silent --show-error \
        --connect-timeout 15 --max-time 90 \
        --proto '=https' \
        --request "$method" \
        --header 'Content-Type: application/json' \
        --data-binary "@$payload_file" \
        --output "$body_file" \
        --write-out '%{http_code}' \
        "$url"
  else
    printf 'header = "JOB-TOKEN: %s"\n' "$CI_JOB_TOKEN" |
      curl --disable --config - \
        --silent --show-error \
        --connect-timeout 15 --max-time 90 \
        --proto '=https' \
        --request "$method" \
        --output "$body_file" \
        --write-out '%{http_code}' \
        "$url"
  fi
}

require_json() {
  jq -e --slurp 'length == 1' "$1" >/dev/null 2>&1 || die "$2 returned malformed JSON"
}

is_missing_release_response() {
  jq -e --slurp '
    length == 1
      and .[0] == {"message": "404 Release Not Found"}
  ' "$1" >/dev/null 2>&1
}

check_tag_and_release_absence() {
  local check_work api tag tag_body status actual_sha access_body release_body
  check_work="$1"
  api="${CI_API_V4_URL%/}/projects/$CI_PROJECT_ID"
  tag="$(encoded_tag)"

  tag_body="$check_work/tag.json"
  if ! status="$(http_request GET "$api/repository/tags/$tag" "$tag_body")"; then
    die "tag consistency request failed (rule=immutable-tags-rule)"
  fi
  [ "$status" = 200 ] || die "tag consistency returned HTTP $status (rule=immutable-tags-rule)"
  require_json "$tag_body" "tag consistency"
  actual_sha="$(jq -r '.commit.id // empty' "$tag_body")"
  [ "$actual_sha" = "$CI_COMMIT_SHA" ] ||
    die "tag commit mismatch (rule=immutable-tags-rule)"

  access_body="$check_work/releases-access.json"
  if ! status="$(http_request GET "$api/releases?per_page=1" "$access_body")"; then
    die "Releases namespace access request failed"
  fi
  [ "$status" = 200 ] || die "Releases namespace access returned HTTP $status"
  require_json "$access_body" "Releases namespace access"
  jq -e 'type == "array"' "$access_body" >/dev/null 2>&1 ||
    die "Releases namespace access returned an unexpected shape"

  release_body="$check_work/release-absence.json"
  if ! status="$(http_request GET "$api/releases/$tag" "$release_body")"; then
    die "release non-overwrite request failed (rule=release-via-merge-request-rule)"
  fi
  case "$status" in
    404)
      require_json "$release_body" "release absence"
      is_missing_release_response "$release_body" ||
        die "release absence returned an unexpected 404 response (rule=release-via-merge-request-rule)"
      ;;
    200) die "release already exists (rule=release-via-merge-request-rule)" ;;
    *) die "release absence returned ambiguous HTTP $status (rule=release-via-merge-request-rule)" ;;
  esac
}

build_payload() {
  local dist_dir payload_file names_file duplicate base links_file count
  dist_dir="$1"
  payload_file="$2"
  [ -d "$dist_dir" ] || die "distribution directory not found"
  [ -n "$payload_file" ] || die "payload path is empty"

  names_file="$work/names"
  "$SCRIPT_DIR/release-assets.sh" list "$dist_dir" >"$names_file"
  [ -s "$names_file" ] || die "release intent contains no asset links"
  duplicate="$(sort "$names_file" | uniq -d | head -n 1)"
  [ -z "$duplicate" ] || die "release intent contains duplicate asset names"

  base="$(package_base)"
  links_file="$work/links.json"
  jq -Rn --arg base "$base" \
    '[inputs | select(length > 0) | {name: ., url: ($base + "/" + .), link_type: "package"}]' \
    <"$names_file" >"$links_file"
  count="$(wc -l <"$names_file" | tr -d ' ')"
  [ "$(jq 'length' "$links_file")" = "$count" ] || die "release link cardinality mismatch"
  jq -e 'length > 0 and ([.[].name] | length == (unique | length)) and ([.[].url] | length == (unique | length))' \
    "$links_file" >/dev/null || die "release link bijection is invalid"

  jq -n \
    --arg tag "$CI_COMMIT_TAG" \
    --arg name "agents-govern $CI_COMMIT_TAG" \
    --arg description "See [CHANGELOG](https://gitlab.com/$CI_PROJECT_PATH/-/blob/$CI_COMMIT_TAG/agf/CHANGELOG.md) for release notes." \
    --slurpfile links "$links_file" \
    '{tag_name: $tag, name: $name, description: $description, assets: {links: $links[0]}}' \
    >"$payload_file"
}

validate_payload() {
  local dist_dir payload_file expected
  dist_dir="$1"
  payload_file="$2"
  [ -f "$payload_file" ] || die "release payload is missing"
  require_json "$payload_file" "release payload"

  expected="$work/expected.json"
  build_payload "$dist_dir" "$expected"
  if ! jq -e --slurp '.[0] == .[1]' "$expected" "$payload_file" >/dev/null; then
    die "release payload no longer matches the verified artifact intent"
  fi
  jq -e 'has("ref") | not' "$payload_file" >/dev/null || die "release payload must omit ref"
}

verify_release() {
  local response_file payload_file
  response_file="$1"
  payload_file="$2"
  require_json "$response_file" "release verification"
  jq -e --arg sha "$CI_COMMIT_SHA" --slurpfile intent "$payload_file" '
    . as $actual
    | $intent[0] as $expected
    | $actual.tag_name == $expected.tag_name
      and $actual.name == $expected.name
      and $actual.description == $expected.description
      and $actual.commit.id == $sha
      and (([$actual.assets.links[] | {name, url, link_type}] | sort_by(.name, .url, .link_type))
        == ([$expected.assets.links[] | {name, url, link_type}] | sort_by(.name, .url, .link_type)))
      and ($actual.assets.links | length) == ($expected.assets.links | length)
  ' "$response_file" >/dev/null 2>&1 || die "created release does not match the intended tag and assets"
}

diagnose_ambiguous_post() {
  local api tag diagnostic state status
  api="${CI_API_V4_URL%/}/projects/$CI_PROJECT_ID"
  tag="$(encoded_tag)"
  diagnostic="$work/ambiguous-release.json"
  state=unknown
  if status="$(http_request GET "$api/releases/$tag" "$diagnostic")"; then
    case "$status" in
      200) state=present ;;
      404)
        if is_missing_release_response "$diagnostic"; then
          state=absent
        fi
        ;;
    esac
  fi
  printf 'release:create indeterminate POST; diagnostic release state=%s; operator reconciliation required\n' "$state" >&2
}

preflight() {
  local dist_dir payload_file
  dist_dir="$1"
  payload_file="$2"
  build_payload "$dist_dir" "$payload_file"
  check_tag_and_release_absence "$work"
}

publish() {
  local dist_dir payload_file api post_body status get_body tag
  dist_dir="$1"
  payload_file="$2"
  validate_payload "$dist_dir" "$payload_file"
  check_tag_and_release_absence "$work"

  api="${CI_API_V4_URL%/}/projects/$CI_PROJECT_ID"
  post_body="$work/post.json"
  if ! status="$(http_request POST "$api/releases" "$post_body" "$payload_file")"; then
    diagnose_ambiguous_post
    die "release POST outcome is indeterminate; POST was not retried"
  fi
  [ "$status" = 201 ] || die "release creation returned HTTP $status"
  verify_release "$post_body" "$payload_file"

  get_body="$work/created-release.json"
  tag="$(encoded_tag)"
  if ! status="$(http_request GET "$api/releases/$tag" "$get_body")"; then
    die "created release follow-up request failed"
  fi
  [ "$status" = 200 ] || die "created release follow-up returned HTTP $status"
  verify_release "$get_body" "$payload_file"
}

require_command jq
require_command curl

command_name="${1-}"
case "$command_name" in
  package-base)
    [ "$#" -eq 1 ] || die "usage: create-release.sh package-base"
    package_base
    ;;
  preflight|publish)
    [ "$#" -eq 3 ] || die "usage: create-release.sh $command_name <dist-dir> <payload-file>"
    require_context
    work="$(mktemp -d)"
    trap 'rm -rf "$work"' EXIT INT TERM
    "$command_name" "$2" "$3"
    ;;
  *)
    die "usage: create-release.sh {package-base|preflight|publish}"
    ;;
esac
