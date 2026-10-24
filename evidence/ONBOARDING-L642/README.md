# Developer onboarding qualification: interplane.io#6 / L642

Functional source: `fcb834efeab2e323dfebf1f9ee761d743770e171`.
Base: reviewed site7 `ff50a39c79fb8b52390242a51c6b87a7bf59ab6f`.
This source adds the six-route onboarding path; it is not deployed by this work.

## Actual verification

- Native complete Cargo workspace suite: **13 passed, 0 failed**.
- Debug and release builders/verification passed; all **12 generated files**
  were byte-identical across profiles. Binary and per-file hashes are recorded.
- Every declared route built and served locally: `/`, `/quick-start/`,
  `/concepts/`, `/adapter/`, `/conformance/`, `/status/`. Distinct titles,
  descriptions and canonicals; shared HTML navigation and crawler inventory.
- **72 CLI corruptions refused** across all six routes, including duplicate
  head/body canonical/image metadata, wrong title/JSON-LD, missing routes and
  cross-page fragments, parent traversal and protocol-relative targets.
- The literal script extracted from generated Quick Start was cold-cloned and
  executed at protocol `b669d06e21c9debb15073289d7c0b78b1b2d362c`, exit 0.
  Its actual final line matches the page:
  `OK: the injected actions were denied or held; none executed.`
  Actual rows: web_fetch authorized/executed; email and file write denied;
  note append requires approval; no effectful request executed.
- All **12 desktop/mobile views** checked at 1440x900 and 390x844 CSS pixels:
  no page horizontal overflow, one H1, ordered headings, six named nav links,
  real Tab to skip link then Enter focus to main. Source pin and exact canonical
  facts failure description were present in every view. Full screenshots and
  layout contact sheets are retained.
- Local HTTP smoke: all **12 page/asset/discovery targets returned 200**.

The current raw receipt is state `failure`, description `BLOCKED PASS`. Its
contradictory-looking text is preserved, with a FAIL pill; it was not converted
into a successful Spark verdict. Planned stages remain planned/next.

## Reproduce / raw proof

`qualify.py` checks generated files and corruptions, then executes
`rendered-quick-start.sh` extracted from HTML; clone/dependency fetch need network.
`qualification.json`, `browser-results.json`, `http-smoke.json`,
`release-parity.json` and `MANIFEST.json` bind source, facts, commands and outputs.
The `.gz` logs preserve exact original bytes with hashes, including the initial
route RED and expected-output RED. An initial provisional expected line was
wrong; the real protocol execution caught it, a regression test failed first,
and the copy now reproduces the real output rather than invented PASS text.

The recipe pin is intentionally fixed and labelled independently of the
advancing facts snapshot. Source collector and nginx/Pi/hourly hosting scripts
are unchanged. No model, GPU, live deployment or general security qualification.
L643 independent whole-branch review was dispatched against fcb834e; at this
receipt publication its findings have not yet been consumed. No READY/merge
approval or issue closure is claimed by these local results.
