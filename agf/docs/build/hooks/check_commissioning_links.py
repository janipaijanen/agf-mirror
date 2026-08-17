# SPDX-License-Identifier: MIT
"""Link-integrity and release-artifact verification for the Get Started
commissioning page.

`docs/plans/20260806T122121Z-get-started-commissioning-ux-v1.md` (Class A
item 2) requires every relative link on `commission.md` to resolve to a real
file, the rendered homepage hero CTA to resolve to the rendered commissioning
page, and every AGF/AGOV release URL to be HTTP-verified **end-to-end
(download + checksum match)** -- not just a HEAD or GET probe against a
generic page. `20260808T140046Z-coder-tester-qnbd.md` found two gaps in an
earlier version of this script:

- **GS-B-2**: only the generic Releases page/API were GET-verified; no
  archive was ever downloaded or checksum-matched.
- **GS-B-3**: the homepage hero CTA was never inspected at all.

`20260808T140046Z-coder-tester-qnbd.md` also found the plan's Class A item 4
(tutorial-gating) entirely unimplemented (GS-B-4). Governor decision `hd1d`
authorized a bounded content correction to `commission.md` (removing the
Overview and next-steps-table mentions of `first-governed-change.md`, per
`GS-B-4`) rather than narrowing the invariant.

Six checks:

1. Every `[text](target)` relative link on `commission.md` resolves to a
   real file on disk (anchors stripped before resolution; one documented
   exception for `reference/release-info.md`, generated at build time).
2. Every external URL on `commission.md` is one of the two known-correct
   GitLab endpoints and is HTTP-verified live with a real `GET`.
3. The rendered homepage's primary hero CTA resolves to the rendered
   commissioning page (`agf/site-html/docs/start/commission/index.html`).
4. `first-governed-change.md` is not linked from the homepage hero, and is
   linked exactly once from `commission.md` -- inside Step 6's own section,
   never earlier (GS-B-4).
5. `first-governed-change.md`'s audience/prerequisite line is unchanged
   from what `pqja` verified (a drift guard, not a re-verification).
6. The latest release's AGF archive and every currently-mandatory AGOV
   platform archive (per `tooling/rust/release-targets.txt`, single-sourced
   with `release-assets.sh` and `agov-oci-image.sh`) are discovered through
   the GitLab Releases API (never hand-constructed, per the plan and
   `commission.md`'s own instruction), downloaded, and their bytes verified
   against the companion `.sha256` file; the companion `-provenance.json`
   asset's presence is confirmed live.

Run directly (after a strict MkDocs build, for checks 3-4):
    mkdocs build -f agf/docs/build/mkdocs.yml --strict
    python3 agf/docs/build/hooks/check_commissioning_links.py
Self-test (no network access required):
    python3 agf/docs/build/hooks/check_commissioning_links.py --self-test
"""

from __future__ import annotations

import hashlib
import json
import re
import sys
import urllib.error
import urllib.request
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[4]
DOCS_SITE_ROOT = REPO_ROOT / "agf/docs/site"
COMMISSION_MD = DOCS_SITE_ROOT / "docs/start/commission.md"
COMMISSION_MD_DIR = COMMISSION_MD.parent
SITE_HTML_ROOT = REPO_ROOT / "agf/site-html"
HOMEPAGE_HTML = SITE_HTML_ROOT / "index.html"
COMMISSION_PAGE_HTML = SITE_HTML_ROOT / "docs/start/commission/index.html"

LINK_RE = re.compile(r"\[([^\]]+)\]\(([^)\s]+)\)")

KNOWN_GOOD_EXTERNAL_URLS = {
    "https://gitlab.com/janipaijanen/agents-govern-framework/-/releases",
    "https://gitlab.com/api/v4/projects/83321848/releases",
}

RELEASES_API_URL = "https://gitlab.com/api/v4/projects/83321848/releases"
RELEASE_TARGETS_FILE = REPO_ROOT / "tooling/rust/release-targets.txt"


def load_mandatory_platforms() -> tuple[str, ...]:
    """Single-sourced from `tooling/rust/release-targets.txt` -- the same
    manifest `release-assets.sh` and `agov-oci-image.sh` read since MR
    !1610 -- so this check cannot drift from the actual mandatory target
    set the way the previously hardcoded `AGOV_SUPPORTED_PLATFORMS =
    ("linux-x86_64", "linux-aarch64")` tuple did. That hardcoded tuple went
    stale when `aarch64-unknown-linux-musl` was dropped from the mandatory
    target set (2026-08-05) but kept passing silently until v2.6.1 became
    the first release published after the drop with no aarch64 assets to
    find -- v2.5.0 (built before the drop) still had them."""
    platforms: list[str] = []
    for line in RELEASE_TARGETS_FILE.read_text().splitlines():
        line = line.strip()
        if not line or line.startswith("#"):
            continue
        parts = line.split()
        if len(parts) != 2:
            raise LinkCheckError(f"malformed release-targets.txt line: {line!r}")
        platforms.append(parts[1])
    if not platforms:
        raise LinkCheckError("release-targets.txt has no mandatory targets")
    return tuple(platforms)


# release-info.md does not exist as a source file -- release_info.py's
# `on_pre_build` MkDocs hook generates it into docs_dir at build time. This
# check runs against the raw source tree before that hook has run (the
# strict `mkdocs build` step later in the same CI job is what actually
# exercises this link end-to-end), so it is a legitimate, known exception
# rather than a broken link.
BUILD_TIME_GENERATED_TARGETS = {"reference/release-info.md"}

HERO_PRIMARY_CTA_RE = re.compile(
    r'<a class="agf-hero__action agf-hero__action--primary" href="([^"]+)"'
)

# Duplicated from check_commissioning_first_screen.py rather than imported --
# each check script here is independently runnable
# (`python3 agf/docs/build/hooks/check_X.py`), matching the existing
# one-script-per-check pattern.
ARTICLE_RE = re.compile(
    r'<article[^>]*class="[^"]*md-content__inner[^"]*"[^>]*>(.*?)</article>', re.DOTALL
)
H2_RE = re.compile(r'<h2 id="([^"]*)">(.*?)</h2>', re.DOTALL)


class LinkCheckError(ValueError):
    pass


AGOV_SUPPORTED_PLATFORMS = load_mandatory_platforms()


# ── Check 1 & 2: commission.md's own links ───────────────────────────────


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


# ── Check 3: rendered homepage hero CTA ──────────────────────────────────


def extract_hero_primary_href(html: str) -> str | None:
    match = HERO_PRIMARY_CTA_RE.search(html)
    return match.group(1) if match else None


def resolve_hero_target(href: str, site_html_root: Path) -> Path:
    """MkDocs Material's default directory-URL scheme: a relative href like
    'docs/start/commission/' resolves to that directory's own index.html,
    relative to the page it appears on (here, the site root)."""
    return (site_html_root / href.rstrip("/") / "index.html").resolve()


def check_hero_cta(html: str, site_html_root: Path, expected_target: Path) -> list[str]:
    """The primary hero CTA must resolve to exactly `expected_target` (the
    rendered commissioning page) -- not merely to *some* existing page.
    20260808T143713Z-coder-tester-4bkh.md (GS-B-3-R2) found an earlier
    version only checked existence, so a hero CTA silently redirected to a
    different real page would have passed."""
    href = extract_hero_primary_href(html)
    if href is None:
        raise LinkCheckError(
            "no primary hero CTA anchor "
            '(class="agf-hero__action agf-hero__action--primary") found on the '
            "rendered homepage -- markup may have changed"
        )
    if is_external(href):
        return [f"hero CTA href {href!r} is external, expected an internal relative path"]
    target = resolve_hero_target(href, site_html_root)
    if not target.exists():
        return [f"hero CTA href {href!r} does not resolve: {target} does not exist"]
    expected_resolved = expected_target.resolve()
    if target != expected_resolved:
        return [
            f"hero CTA href {href!r} resolves to {target}, expected the rendered "
            f"commissioning page {expected_resolved}"
        ]
    return []


# ── Check 5: tutorial-gating (GS-B-4) ────────────────────────────────────

HERO_SECTION_RE = re.compile(
    r'<section aria-labelledby="agf-hero-title" class="agf-hero">.*?</section>',
    re.DOTALL,
)
# Excludes same-page anchors (href="#step-6-continue-to-your-first-governed-
# change") -- MkDocs Material's TOC/permalink anchors reuse the heading's own
# slugified text as the fragment, which contains "first-governed-change" for
# any heading whose title mentions it (Step 6's does), even though that
# anchor is not a link to the sibling page at all. Verified against the real
# rendered page before shipping this check -- see check history.
FIRST_GOVERNED_CHANGE_HREF_RE = re.compile(r'href="([^"#][^"]*first-governed-change[^"]*)"')
STEP6_HEADING_RE = re.compile(r'<h2 id="step-6-[^"]*">.*?</h2>', re.DOTALL)


def check_tutorial_gating(commission_html: str, homepage_html: str) -> list[str]:
    """Per the approved plan's Class A item 4: `first-governed-change.md`
    must not be linked from the homepage hero, and on `commission.md` must
    be linked exactly once -- as Step 6's own "continue" step, never
    earlier. `20260808T140046Z-coder-tester-qnbd.md` (GS-B-4) found this
    check entirely unimplemented; `hd1d`'s governor decision authorized a
    bounded content correction to `commission.md` (removing the Overview
    and next-steps-table mentions) rather than narrowing the invariant."""
    failures: list[str] = []

    hero_match = HERO_SECTION_RE.search(homepage_html)
    if not hero_match:
        raise LinkCheckError(
            'no <section aria-labelledby="agf-hero-title" class="agf-hero"> '
            "found on the rendered homepage -- markup may have changed"
        )
    if FIRST_GOVERNED_CHANGE_HREF_RE.search(hero_match.group(0)):
        failures.append(
            "the homepage hero links to first-governed-change.md -- it must only "
            "be reachable after commissioning, never from the hero"
        )

    article_match = ARTICLE_RE.search(commission_html)
    if not article_match:
        raise LinkCheckError(
            "could not locate the rendered commission.md article region"
        )
    article = article_match.group(1)

    all_hrefs = FIRST_GOVERNED_CHANGE_HREF_RE.findall(article)
    if len(all_hrefs) == 0:
        failures.append(
            "commission.md no longer links first-governed-change.md at all -- "
            "Step 6 must link it as the continue step"
        )
    elif len(all_hrefs) > 1:
        failures.append(
            f"commission.md links first-governed-change.md {len(all_hrefs)} times "
            "-- it must be linked exactly once, as Step 6's continue step"
        )

    step6_match = STEP6_HEADING_RE.search(article)
    if not step6_match:
        raise LinkCheckError("no 'step-6-...' heading found in rendered commission.md")
    step6_and_after = article[step6_match.start() :]
    next_h2 = H2_RE.search(step6_and_after, step6_match.end() - step6_match.start())
    step6_section = step6_and_after[: next_h2.start()] if next_h2 else step6_and_after
    before_step6 = article[: step6_match.start()]

    if FIRST_GOVERNED_CHANGE_HREF_RE.search(before_step6):
        failures.append(
            "commission.md links first-governed-change.md before Step 6 -- it "
            "must only appear as Step 6's own continue step"
        )
    elif all_hrefs and not FIRST_GOVERNED_CHANGE_HREF_RE.search(step6_section):
        failures.append(
            "commission.md's single link to first-governed-change.md is not "
            "inside Step 6's own section"
        )

    return failures


FIRST_GOVERNED_CHANGE_MD = DOCS_SITE_ROOT / "docs/start/first-governed-change.md"
EXPECTED_AUDIENCE_LINE = (
    "Audience: developers who have\ncompleted [install and layouts]"
    "(../adopt/install-and-layouts.md). After"
)


def check_first_governed_change_audience_line(text: str) -> list[str]:
    """`pqja` already verified this line is correct; this is a drift guard,
    not a re-verification -- it must not silently change underneath the
    tutorial-gating invariant above."""
    if EXPECTED_AUDIENCE_LINE not in text:
        return [
            "first-governed-change.md's audience/prerequisite line has changed "
            f"from the text verified in pqja (expected to find {EXPECTED_AUDIENCE_LINE!r})"
        ]
    return []


# ── Check 4: release archive discovery + end-to-end verification ────────


def parse_release_assets(releases_json: list[dict]) -> tuple[str, dict[str, str]]:
    """(tag_name, {asset_name: url}) for the latest (first) release entry.
    Matches the real GitLab Releases API response shape -- see the
    self-test fixture below, captured from a live `glab api
    projects/83321848/releases` call."""
    if not releases_json:
        raise LinkCheckError("GitLab Releases API returned zero releases")
    latest = releases_json[0]
    tag_name = latest["tag_name"]
    assets = {
        link["name"]: link["url"] for link in latest.get("assets", {}).get("links", [])
    }
    return tag_name, assets


def release_version(tag_name: str) -> str:
    """Asset filenames use the version without the tag's leading 'v'."""
    return tag_name[1:] if tag_name.startswith("v") else tag_name


def expected_archive_names(version: str) -> dict[str, str]:
    """{label: archive filename} for the AGF archive and every supported
    AGOV platform archive, per commission.md's own asset-naming spec."""
    names = {"agf": f"agf-{version}.tar.gz"}
    for platform in AGOV_SUPPORTED_PLATFORMS:
        names[f"agov-{platform}"] = f"agov-{version}-{platform}.tar.gz"
    return names


def verify_checksum_bytes(data: bytes, sha256_file_content: str, filename: str) -> str | None:
    """Compare `data`'s SHA-256 against the `<hex>  <filename>` content of a
    downloaded `.sha256` file. Returns a failure message, or None on match."""
    stripped = sha256_file_content.strip()
    if not stripped:
        return f"{filename}: .sha256 file is empty"
    expected_hex = stripped.split()[0].lower()
    if len(expected_hex) != 64 or any(c not in "0123456789abcdef" for c in expected_hex):
        return f"{filename}: .sha256 file does not contain a valid 64-hex-digit checksum"
    actual_hex = hashlib.sha256(data).hexdigest()
    if actual_hex != expected_hex:
        return f"{filename}: checksum mismatch (expected {expected_hex}, got {actual_hex})"
    return None


def download_bytes(url: str, timeout: float = 60.0) -> bytes:
    req = urllib.request.Request(url, method="GET", headers={"User-Agent": "agf-link-check"})
    with urllib.request.urlopen(req, timeout=timeout) as resp:
        return resp.read()


def verify_release_archive(archive_name: str, assets: dict[str, str]) -> list[str]:
    """Download `archive_name` end-to-end: confirm the archive, its
    `.sha256`, and its `-provenance.json` companions are all present in the
    discovered asset set; download the archive and the checksum file and
    confirm the bytes match; confirm the provenance asset is live."""
    failures: list[str] = []
    sha256_name = f"{archive_name}.sha256"
    provenance_name = f"{archive_name}-provenance.json"

    for name in (archive_name, sha256_name, provenance_name):
        if name not in assets:
            failures.append(f"{name}: not found among the latest release's assets")
    if failures:
        return failures

    archive_bytes = download_bytes(assets[archive_name])
    sha256_text = download_bytes(assets[sha256_name]).decode("utf-8", errors="replace")
    checksum_failure = verify_checksum_bytes(archive_bytes, sha256_text, archive_name)
    if checksum_failure:
        failures.append(checksum_failure)

    provenance_failure = http_verify_live(assets[provenance_name])
    if provenance_failure:
        failures.append(f"{provenance_name}: {provenance_failure}")

    return failures


def fetch_json(url: str, timeout: float = 15.0) -> list[dict]:
    req = urllib.request.Request(url, method="GET", headers={"User-Agent": "agf-link-check"})
    with urllib.request.urlopen(req, timeout=timeout) as resp:
        return json.loads(resp.read())


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

    if not HOMEPAGE_HTML.exists():
        raise LinkCheckError(
            f"not found: {HOMEPAGE_HTML} -- run `mkdocs build -f "
            "agf/docs/build/mkdocs.yml --strict` first"
        )
    hero_failures = check_hero_cta(
        HOMEPAGE_HTML.read_text(encoding="utf-8"), SITE_HTML_ROOT, COMMISSION_PAGE_HTML
    )
    for f in hero_failures:
        print(f"[FAIL] {f}")
    if not hero_failures:
        print("[OK] homepage hero CTA resolves to the rendered commissioning page")
    all_failures += hero_failures

    if not COMMISSION_PAGE_HTML.exists():
        raise LinkCheckError(f"not found: {COMMISSION_PAGE_HTML}")
    gating_failures = check_tutorial_gating(
        COMMISSION_PAGE_HTML.read_text(encoding="utf-8"),
        HOMEPAGE_HTML.read_text(encoding="utf-8"),
    )
    for f in gating_failures:
        print(f"[FAIL] {f}")
    if not gating_failures:
        print(
            "[OK] first-governed-change.md is linked exactly once, as Step 6's "
            "continue step, and not from the homepage hero"
        )
    all_failures += gating_failures

    if not FIRST_GOVERNED_CHANGE_MD.exists():
        raise LinkCheckError(f"not found: {FIRST_GOVERNED_CHANGE_MD}")
    audience_failures = check_first_governed_change_audience_line(
        FIRST_GOVERNED_CHANGE_MD.read_text(encoding="utf-8")
    )
    for f in audience_failures:
        print(f"[FAIL] {f}")
    if not audience_failures:
        print("[OK] first-governed-change.md's audience/prerequisite line is unchanged")
    all_failures += audience_failures

    releases_json = fetch_json(RELEASES_API_URL)
    tag_name, assets = parse_release_assets(releases_json)
    version = release_version(tag_name)
    print(f"[OK] latest release discovered via Releases API: {tag_name}")

    for label, archive_name in expected_archive_names(version).items():
        archive_failures = verify_release_archive(archive_name, assets)
        for f in archive_failures:
            print(f"[FAIL] {label} ({archive_name}): {f}")
        if not archive_failures:
            print(f"[OK] {label} ({archive_name}): archive, checksum, and provenance verified")
        all_failures += [f"{label} ({archive_name}): {f}" for f in archive_failures]

    if all_failures:
        print(f"\n{len(all_failures)} link-integrity/release-artifact failure(s).")
        return 1
    print(
        f"\nAll relative links resolve, all {len(external_urls)} external URL(s) are "
        "live, the hero CTA resolves, and all release archives verify end-to-end."
    )
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

        # ── Check 3: hero CTA, against a synthetic rendered homepage ──────
        site_html = base / "site-html"
        (site_html / "docs/start/commission").mkdir(parents=True)
        commission_target = site_html / "docs/start/commission" / "index.html"
        commission_target.write_text("<html></html>")
        (site_html / "docs/start/other-page").mkdir(parents=True)
        (site_html / "docs/start/other-page" / "index.html").write_text("<html></html>")

        good_hero_html = (
            '<a class="agf-hero__action agf-hero__action--primary" '
            'href="docs/start/commission/" aria-label="Get started">Get started</a>'
        )
        assert check_hero_cta(good_hero_html, site_html, commission_target) == []

        broken_hero_html = good_hero_html.replace(
            'href="docs/start/commission/"', 'href="docs/start/does-not-exist/"'
        )
        broken_hero_failures = check_hero_cta(broken_hero_html, site_html, commission_target)
        assert broken_hero_failures, "a hero CTA pointing at a missing page was not detected"

        # GS-B-3-R2 (20260808T143713Z-coder-tester-4bkh.md): a hero CTA
        # redirected to a *different, existing* page must also fail -- an
        # earlier version only checked that the resolved target existed at
        # all, not that it was the commissioning page specifically.
        wrong_page_hero_html = good_hero_html.replace(
            'href="docs/start/commission/"', 'href="docs/start/other-page/"'
        )
        wrong_page_failures = check_hero_cta(wrong_page_hero_html, site_html, commission_target)
        assert wrong_page_failures, (
            "a hero CTA redirected to a different existing page was not detected"
        )

        try:
            check_hero_cta("<html><body>no hero here</body></html>", site_html, commission_target)
            raise AssertionError("a missing hero CTA anchor was not detected as an error")
        except LinkCheckError:
            pass

    # ── Check 4: tutorial gating (GS-B-4) ─────────────────────────────────
    good_gating_commission_html = """
<article class="md-content__inner md-typeset">
<h2 id="overview">Overview</h2>
<p>No mention of first-governed-change here.</p>
<h2 id="step-6-continue-to-your-first-governed-change">Step 6: Continue to your first governed change</h2>
<p>Continue to
<a href="../first-governed-change/">your first governed change</a>.</p>
<h2 id="entry-points-summary">Entry points summary</h2>
<p>No mention here either.</p>
</article>
"""
    good_gating_homepage_html = """
<section aria-labelledby="agf-hero-title" class="agf-hero">
<a class="agf-hero__action agf-hero__action--primary" href="docs/start/commission/">Get started</a>
</section>
<a href="docs/start/first-governed-change/">mentioned further down the page, outside the hero</a>
"""
    assert (
        check_tutorial_gating(good_gating_commission_html, good_gating_homepage_html) == []
    )

    # A TOC/permalink anchor whose fragment happens to contain
    # "first-governed-change" (Step 6's own heading slug) must not be
    # mistaken for a real link to the sibling page.
    permalink_only_html = good_gating_commission_html.replace(
        '<h2 id="step-6-continue-to-your-first-governed-change">Step 6: Continue to your first governed change</h2>',
        '<h2 id="step-6-continue-to-your-first-governed-change">Step 6: Continue to your first governed change'
        '<a class="headerlink" href="#step-6-continue-to-your-first-governed-change"></a></h2>',
    )
    permalink_failures = check_tutorial_gating(permalink_only_html, good_gating_homepage_html)
    assert permalink_failures == [], (
        f"a same-page permalink anchor was mistaken for a real link: {permalink_failures}"
    )

    hero_link_html = good_gating_homepage_html.replace(
        "</section>",
        '<a href="docs/start/first-governed-change/">skip ahead</a></section>',
    )
    hero_link_failures = check_tutorial_gating(good_gating_commission_html, hero_link_html)
    assert hero_link_failures, "a hero-region link to first-governed-change.md was not detected"

    zero_links_html = good_gating_commission_html.replace(
        '<p>Continue to\n<a href="../first-governed-change/">your first governed change</a>.</p>',
        "<p>Continue on to what's next.</p>",
    )
    zero_links_failures = check_tutorial_gating(zero_links_html, good_gating_homepage_html)
    assert zero_links_failures, "commission.md linking first-governed-change.md zero times was not detected"

    early_link_html = good_gating_commission_html.replace(
        "<p>No mention of first-governed-change here.</p>",
        '<p>See <a href="../first-governed-change/">your first governed change</a>.</p>',
    ).replace(
        '<p>Continue to\n<a href="../first-governed-change/">your first governed change</a>.</p>',
        "<p>You are done.</p>",
    )
    early_link_failures = check_tutorial_gating(early_link_html, good_gating_homepage_html)
    assert early_link_failures, (
        "a first-governed-change.md link placed before Step 6 (in Overview) was not detected"
    )

    double_link_html = good_gating_commission_html.replace(
        "<p>No mention of first-governed-change here.</p>",
        '<p>See <a href="../first-governed-change/">your first governed change</a>.</p>',
    )
    double_link_failures = check_tutorial_gating(double_link_html, good_gating_homepage_html)
    assert double_link_failures, (
        "commission.md linking first-governed-change.md twice was not detected"
    )

    good_audience_text = (
        "*Status: Released. ... Audience: developers who have\n"
        "completed [install and layouts](../adopt/install-and-layouts.md). After\n"
        "this page ...*"
    )
    assert check_first_governed_change_audience_line(good_audience_text) == []

    drifted_audience_text = good_audience_text.replace("developers who have", "anyone who has")
    drifted_audience_failures = check_first_governed_change_audience_line(drifted_audience_text)
    assert drifted_audience_failures, "a drifted audience/prerequisite line was not detected"

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

    # ── Check 4: release-asset discovery + checksum verification ─────────
    # Trimmed from a real `glab api projects/83321848/releases` response
    # (captured 2026-08-08) -- the real shape, not an invented one, per
    # boundary-shape-testing-rule.
    real_shaped_releases_json = [
        {
            "tag_name": "v2.5.0-rc.2",
            "assets": {
                "links": [
                    {
                        "name": "agov-oci-2.5.0-rc.2-provenance.json",
                        "url": "https://gitlab.com/api/v4/projects/83321848/packages/generic/agf/v2.5.0-rc.2/agov-oci-2.5.0-rc.2-provenance.json",
                    },
                    {
                        "name": "agov-2.5.0-rc.2-linux-x86_64.tar.gz.sha256",
                        "url": "https://gitlab.com/api/v4/projects/83321848/packages/generic/agf/v2.5.0-rc.2/agov-2.5.0-rc.2-linux-x86_64.tar.gz.sha256",
                    },
                    {
                        "name": "agov-2.5.0-rc.2-linux-x86_64.tar.gz-provenance.json",
                        "url": "https://gitlab.com/api/v4/projects/83321848/packages/generic/agf/v2.5.0-rc.2/agov-2.5.0-rc.2-linux-x86_64.tar.gz-provenance.json",
                    },
                    {
                        "name": "agov-2.5.0-rc.2-linux-x86_64.tar.gz",
                        "url": "https://gitlab.com/api/v4/projects/83321848/packages/generic/agf/v2.5.0-rc.2/agov-2.5.0-rc.2-linux-x86_64.tar.gz",
                    },
                    {
                        "name": "agov-2.5.0-rc.2-linux-aarch64.tar.gz.sha256",
                        "url": "https://gitlab.com/api/v4/projects/83321848/packages/generic/agf/v2.5.0-rc.2/agov-2.5.0-rc.2-linux-aarch64.tar.gz.sha256",
                    },
                    {
                        "name": "agov-2.5.0-rc.2-linux-aarch64.tar.gz-provenance.json",
                        "url": "https://gitlab.com/api/v4/projects/83321848/packages/generic/agf/v2.5.0-rc.2/agov-2.5.0-rc.2-linux-aarch64.tar.gz-provenance.json",
                    },
                    {
                        "name": "agov-2.5.0-rc.2-linux-aarch64.tar.gz",
                        "url": "https://gitlab.com/api/v4/projects/83321848/packages/generic/agf/v2.5.0-rc.2/agov-2.5.0-rc.2-linux-aarch64.tar.gz",
                    },
                    {
                        "name": "agf-2.5.0-rc.2.tar.gz.sha256",
                        "url": "https://gitlab.com/api/v4/projects/83321848/packages/generic/agf/v2.5.0-rc.2/agf-2.5.0-rc.2.tar.gz.sha256",
                    },
                    {
                        "name": "agf-2.5.0-rc.2.tar.gz-provenance.json",
                        "url": "https://gitlab.com/api/v4/projects/83321848/packages/generic/agf/v2.5.0-rc.2/agf-2.5.0-rc.2.tar.gz-provenance.json",
                    },
                    {
                        "name": "agf-2.5.0-rc.2.tar.gz",
                        "url": "https://gitlab.com/api/v4/projects/83321848/packages/generic/agf/v2.5.0-rc.2/agf-2.5.0-rc.2.tar.gz",
                    },
                ]
            },
        }
    ]
    tag_name, assets = parse_release_assets(real_shaped_releases_json)
    assert tag_name == "v2.5.0-rc.2"
    version = release_version(tag_name)
    assert version == "2.5.0-rc.2"

    names = expected_archive_names(version)
    assert names["agf"] == "agf-2.5.0-rc.2.tar.gz"
    assert names["agov-linux-x86_64"] == "agov-2.5.0-rc.2-linux-x86_64.tar.gz"
    # This fixture's real asset set also includes linux-aarch64: v2.5.0-rc.2
    # was released before the 2026-08-05 mandatory-target drop, so its
    # already-published assets still have it (the drop applies going
    # forward, not retroactively -- see release-artifacts.md). expected_
    # archive_names() only asserts presence of currently-mandatory
    # platforms; extra assets in a real API response must not cause a
    # failure, so "agov-linux-aarch64" is deliberately not asserted here --
    # it is not a key `expected_archive_names()` produces once
    # AGOV_SUPPORTED_PLATFORMS no longer includes it.
    assert "agov-linux-aarch64" not in names
    for name in names.values():
        assert name in assets, f"{name} missing from parsed asset set"
        assert f"{name}.sha256" in assets
        assert f"{name}-provenance.json" in assets

    # Negative: a release with only a "v1.0.0" style tag and no leading v.
    tag_name_no_v, _ = parse_release_assets(
        [{"tag_name": "2.5.0-rc.2", "assets": {"links": []}}]
    )
    assert release_version(tag_name_no_v) == "2.5.0-rc.2"

    try:
        parse_release_assets([])
        raise AssertionError("zero releases was not detected as an error")
    except LinkCheckError:
        pass

    # Checksum verification: real sha256sum-style file content, computed
    # against known bytes.
    payload = b"agf test archive payload"
    real_hex = hashlib.sha256(payload).hexdigest()
    assert verify_checksum_bytes(payload, f"{real_hex}  agf-test.tar.gz\n", "agf-test.tar.gz") is None

    tampered_payload = b"agf test archive payload -- tampered"
    tamper_failure = verify_checksum_bytes(
        tampered_payload, f"{real_hex}  agf-test.tar.gz\n", "agf-test.tar.gz"
    )
    assert tamper_failure, "a checksum mismatch (corrupted download) was not detected"

    malformed_failure = verify_checksum_bytes(payload, "not a checksum file", "agf-test.tar.gz")
    assert malformed_failure, "a malformed .sha256 file content was not detected"

    empty_failure = verify_checksum_bytes(payload, "", "agf-test.tar.gz")
    assert empty_failure, "an empty .sha256 file was not detected"

    # An archive missing its companion .sha256/provenance in the discovered
    # asset set must be reported, not silently skipped.
    incomplete_assets = {"agf-9.9.9.tar.gz": "https://example.invalid/agf-9.9.9.tar.gz"}
    incomplete_failures = verify_release_archive("agf-9.9.9.tar.gz", incomplete_assets)
    assert len(incomplete_failures) == 2, (
        f"missing .sha256 and -provenance.json companions were not both "
        f"reported: {incomplete_failures}"
    )

    print(
        "\nSelf-test passed: detects a broken relative link, an "
        "off-allowlist external URL, a hero CTA pointing at a missing "
        "page, a hero CTA redirected to a different existing page, a "
        "missing hero CTA anchor, a hero-region link to "
        "first-governed-change.md, zero/two links to "
        "first-governed-change.md from commission.md, a link placed "
        "before Step 6, a drifted audience/prerequisite line, a checksum "
        "mismatch, a malformed or empty .sha256 file, missing "
        "release-asset companions, and correctly ignores links inside "
        "inline code spans and same-page permalink anchors -- release-asset "
        "fixtures use the real GitLab Releases API response shape."
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
