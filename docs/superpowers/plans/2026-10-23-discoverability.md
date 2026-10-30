# Interplane discoverability implementation plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans inline. Steps use checkboxes.

**Goal:** Satisfy interplane.io#7 on the current single-page generator without creating issue#6 routes or deploying.
**Architecture:** Reuse Rust/Maud/serde_json and existing Facts. Generate inert structured data, llms.txt and honest crawler files; verify the built outputs and fail closed on script/data corruption. Preserve design and all factual verdicts.
**Tech Stack:** Existing Rust site-builder, Maud, serde_json, browser smoke tests.
**Spec:** https://github.com/aien-dev/interplane.io/issues/7 (read body and empty thread).

## Global constraints
- No executable JavaScript, remote fonts/trackers, deployment, new routes, changed factual collector, or invented ranking claims.
- Existing schema-only script must be inert application/ld+json, strictly parsed and safely escaped. Existing no-script policy becomes no executable scripts.
- Do not fabricate content dates. Omit sitemap lastmod rather than treating hourly fetch time as content change.
- Current page only: route expansion and distinct future doc metadata remain issue#6 owned.

## Review focus
- Facts strings containing HTML/script delimiters must not escape JSON-LD.
- Non-JSON scripts and malformed/schema-mismatched metadata must fail verification.
- Missing crawler files, wrong canonical/image and broken local targets must fail.
- A failed/absent protocol verdict must stay failed/absent in all visible output.
- Small/mobile and keyboard use must retain landmarks, heading order and no horizontal overflow.

### Task 1: Qualified discoverability output
**Files:** Modify crates/site-builder/src/main.rs, page.rs, README.md; create focused discovery.rs if needed. Record raw qualification under evidence/discoverability-issue7/.
**Interfaces:** Consume Facts and page constants. Produce structured_data(Facts), llms(Facts), sitemap(), and verify-built discovery checks called from existing build/verify.
- [ ] Write failing tests for structured-data graph/provenance, malicious JSON strings, inert-only script policy, crawler output/lastmod, local target and corruption checks.
- [ ] Run tests; expected failure for missing metadata/crawler output, not compilation/setup errors.
- [ ] Implement safe JSON serialization and exact metadata generation. Website+SoftwareSourceCode graph only; no rating/security/speed/support claims.
- [ ] Generate llms and sitemap from current actual routes plus pinned existing protocol documents. Add shell code language/accessible label, preserve layout.
- [ ] Extend verify to parse metadata and enforce crawler/local target consistency; test mutations for executable script, malformed JSON, wrong graph/canonical/image, missing llms and target.
- [ ] Run cargo test --locked, real facts/build/verify, diff-check, targeted formatting, and CLI corrupted-output failures. Preserve baseline and raw exit codes.
- [ ] Run desktop/mobile browser/keyboard smoke and inspect screenshots; record limitations honestly.
- [ ] Commit/push source+raw evidence, independent whole-diff review, one fix pass for important findings. READY to sole Renda; do not close issue without integrated acceptance.
