# Summary

Fix `release:create` CI job: install `curl` before uploading archives.

# Scope

- `.gitlab-ci.yml` `release:create` job: add `apk add --no-cache curl`
  as the first script step.

# Why

The `release-cli:latest` image is Alpine-based and does not include `curl`.
The upload loop uses `curl --upload-file` to PUT archives to the GitLab
generic package registry. All other tools used in the script (`ls`, `head`,
`basename`, `xargs`, `sh`) are BusyBox standard utils that ship with Alpine.

# Verification

- CI-only change; no Elixir code modified.

# Out of Scope

No behavior changes beyond unblocking the upload step.
