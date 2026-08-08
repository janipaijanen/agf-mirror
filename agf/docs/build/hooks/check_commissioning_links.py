# SPDX-License-Identifier: MIT
"""Link-integrity check for the Get Started commissioning page.

`docs/plans/20260806T122121Z-get-started-commissioning-ux-v1.md` requires
every relative link on `commission.md` to resolve to a real file, and every
external URL to be HTTP-verified end-to-end rather than assumed -- the
`B-MR1193-CA-3` lesson from the earlier README slice, where a hand-written
release-download URL 404'd and was only caught by a live HTTP probe, not by
MkDocs' own strict internal-link check (which does not see external URLs at
all).

Two checks:

1. Every `[text](target)` relative link on `commission.md` resolves to a
   real file on disk (anchors stripped before resolution).
2. Every external URL on the page is one of the two known-correct GitLab
   endpoints (protects against a similar-looking but wrong project ID or
   path silently still returning 200 on some other page) and, when network
   access is available, is HTTP-verified live with a real `GET`, not a
   `HEAD` probe.

Run directly: `python3 agf/docs/build/hooks/check_commissioning_links.py`
Self-test (no network access required):
`python3 agf/docs/build/hooks/check_commissioning_links.py --self-test`
"""

from __future__ import annotations

import re
import sys
import urllib.error
import urllib.request
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[4]
DOCS_SITE_ROOT = REPO_ROOT / "agf/docs/site"
COMMISSION_MD = DOCS_SITE_ROOT / "docs/start/commission.md"
COMMISSION_MD_DIR = COMMISSION_MD.parent

LINK_RE = re.compile(r"\[([^\]]+)\]\(([^)\s]+)\)")

KNOWN_GOOD_EXTERNAL_URLS = {
    "https://gitlab.com/janipaijanen/agents-govern-framework/-/releases",
    "https://gitlab.com/api/v4/projects/83321848/releases",
}

# release-info.md does not exist as a source file -- release_info.py's
# `on_pre_build` MkDocs hook generates it into docs_dir at build time. This
# check runs against the raw source tree before that hook has run (the
# strict `mkdocs build` step later in the same CI job is what actually
# exercises this link end-to-end), so it is a legitimate, known exception
# rather than a broken link.
BUILD_TIME_GENERATED_TARGETS = {"reference/release-info.md"}


class LinkCheckError(ValueError):
    pass


def extract_links(text: str) -> list[tuple[str, str]]:
    """(link text, target) for every markdown link, code spans excluded."""
    without_code_spans = re.sub(r"`[^`]*`", "", text)
    return LINK_RE.findall(without_code_spans)


def is_external(target: str) -> bool:
    return target.startswith(("http://", "https://"))


def check_relative_links(
    text: str, base_dir: Path, docs_site_root: Path | None = None
) -> list[str]:
    failures: list[str] = []
    generated = (
        {(docs_site_root / t).resolve() for t in BUILD_TIME_GENERATED_TARGETS}
        if docs_site_root
        else set()
    )
    for link_text, target in extract_links(text):
        if is_external(target):
            continue
        path_part = target.split("#", 1)[0]
        if not path_part:
            continue  # pure in-page anchor
        resolved = (base_dir / path_part).resolve()
        if resolved in generated:
            continue
        if not resolved.exists():
            failures.append(
                f"relative link [{link_text}]({target}) does not resolve: "
                f"{resolved} does not exist"
            )
    return failures


def check_external_url_allowlist(text: str) -> tuple[list[str], set[str]]:
    """Return (failures, the set of external URLs actually found)."""
    failures: list[str] = []
    found: set[str] = set()
    for link_text, target in extract_links(text):
        if not is_external(target):
            continue
        found.add(target)
        if target not in KNOWN_GOOD_EXTERNAL_URLS:
            failures.append(
                f"external link [{link_text}]({target}) is not one of the "
                f"known-correct GitLab endpoints {sorted(KNOWN_GOOD_EXTERNAL_URLS)}"
            )
    return failures, found


def http_verify_live(url: str, timeout: float = 15.0) -> str | None:
    """Real GET (not HEAD), matching the B-MR1193-CA-3 lesson. Returns a
    failure message, or None if the URL is live and returns a real body."""
    req = urllib.request.Request(url, method="GET", headers={"User-Agent": "agf-link-check"})
    try:
        with urllib.request.urlopen(req, timeout=timeout) as resp:
            status = resp.status
            body = resp.read(2048)
    except urllib.error.HTTPError as exc:
        return f"{url}: HTTP {exc.code}"
    except urllib.error.URLError as exc:
        return f"{url}: network error ({exc.reason})"
    if status != 200:
        return f"{url}: HTTP {status}"
    if not body:
        return f"{url}: HTTP 200 but empty body"
    return None


def run_real_checks() -> int:
    if not COMMISSION_MD.exists():
        raise LinkCheckError(f"not found: {COMMISSION_MD}")

    text = COMMISSION_MD.read_text(encoding="utf-8")
    all_failures: list[str] = []

    relative_failures = check_relative_links(text, COMMISSION_MD_DIR, DOCS_SITE_ROOT)
    for f in relative_failures:
        print(f"[FAIL] {f}")
    if not relative_failures:
        print("[OK] all relative links on commission.md resolve to real files")
    all_failures += relative_failures

    allowlist_failures, external_urls = check_external_url_allowlist(text)
    for f in allowlist_failures:
        print(f"[FAIL] {f}")
    all_failures += allowlist_failures

    if not external_urls:
        raise LinkCheckError(
            "found zero external URLs on commission.md -- parser is broken "
            "(the page documents two live GitLab endpoints today)"
        )

    for url in sorted(external_urls):
        failure = http_verify_live(url)
        if failure:
            print(f"[FAIL] {failure}")
            all_failures.append(failure)
        else:
            print(f"[OK] {url}: live, HTTP 200")

    if all_failures:
        print(f"\n{len(all_failures)} link-integrity failure(s).")
        return 1
    print(f"\nAll relative links resolve and all {len(external_urls)} external URL(s) are live.")
    return 0


# ── Self-test: no network access required ────────────────────────────────


def _self_test() -> int:
    good_text = """
See [Release information](../../reference/release-info.md) and
[your first governed change](first-governed-change.md).

Download from the
[project GitLab Releases page](https://gitlab.com/janipaijanen/agents-govern-framework/-/releases)
or the
[GitLab Releases API](https://gitlab.com/api/v4/projects/83321848/releases).
"""
    links = extract_links(good_text)
    assert ("Release information", "../../reference/release-info.md") in links
    assert (
        "project GitLab Releases page",
        "https://gitlab.com/janipaijanen/agents-govern-framework/-/releases",
    ) in links

    # Build a throwaway fixture tree so the relative-link check has a real
    # target to resolve against, without touching the actual site.
    import tempfile

    with tempfile.TemporaryDirectory() as tmp:
        base = Path(tmp)
        (base / "reference").mkdir()
        (base / "reference" / "release-info.md").write_text("x")
        (base / "start").mkdir()
        (base / "start" / "first-governed-change.md").write_text("x")

        fixture_text = (
            "See [Release information](../reference/release-info.md) and "
            "[first governed change](first-governed-change.md)."
        )
        good_failures = check_relative_links(fixture_text, base / "start")
        assert good_failures == [], good_failures

        broken_text = fixture_text.replace(
            "first-governed-change.md", "does-not-exist.md"
        )
        broken_failures = check_relative_links(broken_text, base / "start")
        assert broken_failures, "a broken relative link was not detected"

        # A link to a known build-time-generated target (release-info.md)
        # must not be flagged, even though it does not exist as a source
        # file on disk before `mkdocs build` runs.
        generated_link_text = "See [Release information](../reference/release-info.md)."
        generated_failures = check_relative_links(
            generated_link_text, base / "start", docs_site_root=base
        )
        assert generated_failures == [], (
            f"a link to the known build-time-generated release-info.md was "
            f"incorrectly flagged: {generated_failures}"
        )

    good_allowlist_failures, good_urls = check_external_url_allowlist(good_text)
    assert good_allowlist_failures == [], good_allowlist_failures
    assert good_urls == KNOWN_GOOD_EXTERNAL_URLS, good_urls

    bad_url_text = good_text.replace(
        "https://gitlab.com/api/v4/projects/83321848/releases",
        "https://gitlab.com/api/v4/projects/99999999/releases",
    )
    bad_allowlist_failures, _ = check_external_url_allowlist(bad_url_text)
    assert bad_allowlist_failures, "a wrong project-ID URL was not detected"

    # A link inside inline code (a shown-as-example URL, not a real link)
    # must not be extracted as a markdown link at all.
    code_span_text = "Example: `[not a link](https://example.invalid)`."
    assert extract_links(code_span_text) == []

    print(
        "\nSelf-test passed: detects a broken relative link, an "
        "off-allowlist external URL, and correctly ignores links inside "
        "inline code spans."
    )
    return 0


def main(argv: list[str]) -> int:
    if "--self-test" in argv:
        try:
            return _self_test()
        except AssertionError as exc:
            print(f"SELF-TEST FAILURE: {exc}")
            return 1

    try:
        return run_real_checks()
    except LinkCheckError as exc:
        print(f"ERROR: {exc}")
        return 1


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
