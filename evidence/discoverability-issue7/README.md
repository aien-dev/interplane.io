# Discoverability qualification for interplane.io issue 7

This is an undeployed source candidate. It improves discoverability, not measured search ranking. It does not create the developer routes owned by issue 6 or change deployment/visibility.

## What changed

- Inert `application/ld+json` graph for WebSite and SoftwareSourceCode, using the same protocol pin/description as the visible page. No release, security, speed or universal-support claims were added.
- JSON raw-text delimiters are escaped before embedding. Executable/additional scripts, malformed JSON and metadata inconsistent with facts fail verification.
- Generated `llms.txt` links the current homepage, facts and existing pinned protocol documentation. The sitemap lists the actual single route; the old hourly-fetch-derived `lastmod` was removed because it was not a content-change date.
- Verification checks crawler files, canonical/image identity, local files and fragments. Shell code has a language/accessible label; the skip link now focuses the main landmark.
- Existing Rust generator/dependencies, stylesheet, hero image, factual collector and deployment stay in place. JSON-LD does not execute JavaScript or make requests.

## Replay on macOS

From the repository root, with Rust/Cargo and authenticated gh available:

```sh
TMPDIR="$HOME/.hermes/cache/scratch" cargo test --locked -p site-builder
cargo run --locked -p site-builder -- facts facts.json
cargo run --locked -p site-builder -- build dist facts.json
cargo run --locked -p site-builder -- verify dist
python3 evidence/discoverability-issue7/qualify.py
```

`qualify.py` is an external test oracle, not website runtime. It parses built HTML/XML, validates pinned source paths against the authentic Git tree, then runs actual CLI negative controls. It temporarily mutates only the generated local index and restores the original bytes in `finally`.

## Evidence

- `red.log`: initial new-test run; metadata failure was genuine, the build test also had a Cargo working-directory setup error. This setup error is not counted as a behavioral RED.
- `red-fixture-corrected.log`: both new tests fail for absent JSON-LD/llms after fixing that setup.
- `skip-focus-red.log`: missing focusable main assertion fails before the accessibility fix.
- `final-tests.log`, `build.log`, `verify.log`: complete successful commands after the changes.
- `facts.json`, `protocol-tree.json`: authentic retrieved public protocol record, including its **failure** verification state; no passing protocol verdict was invented.
- `qualification.json`: real CLI positive/negative exits and source-link evidence.
- `desktop-dom.json`, `mobile-dom.json`, screenshots: local Chrome smoke. One JSON-LD script, only same-origin stylesheet/image requests, named links/images, ordered headings, no mobile horizontal overflow; Tab/Enter skip navigation focuses `main`.

The initial setup run used Cargo's inherited system temporary directory; subsequent successful tests explicitly use the Hermes scratch TMPDIR. Screenshots preserve the existing design, not a redesign. Browser tooling may add a title marker; generated HTML remains authoritative.

## Independent review and correction

L632 independently ran the original 11-test suite and found a real Important false-negative: duplicate conflicting canonical/image tags were accepted beside the expected tags. The parent reproduced it as a failing regression, then made one corrective pass. Verification now requires the exact facts-bound generated head and matching whole-document metadata-tag counts. Eight head/body, canonical/image and case/quote variants are refused; the original reviewer's scratch input now exits 1 while the authentic output still exits 0. See `review-duplicate-red.log.gz`, `review-correction-tests.log.gz` and `review-correction.json`. This is test-backed parent disposition of the returned review, not a claim that a second independent review ran.

Raw logs are stored as `.log.gz` to preserve their exact original bytes, including terminal trailing blank lines, without producing patch-whitespace errors. `raw-log-storage.json` records their original SHA256 values; decompression was verified byte-for-byte. No historical receipt text was rewritten. `source-artifact-manifest.json` identifies the original reviewed source; `final-source-artifact-manifest.json` identifies the corrected source and rebuilt binary. Generated HTML is unchanged by the verifier-only correction, so the original browser receipts remain applicable.

## Limits

This is accessibility/mobile smoke testing, not a full WCAG certification. JSON-LD verification is a generated-output invariant, not a general untrusted-HTML sanitizer. One current route is qualified; future developer pages need unique per-route metadata when issue 6 adds them. No Markdown mirror was added: the site is already static readable HTML and pinned canonical Markdown remains upstream. Public deployment, live HTTP crawler endpoints, integrated review and organic search effects are separate from this candidate.
