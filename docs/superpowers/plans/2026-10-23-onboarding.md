# Developer onboarding implementation plan

> **For agentic workers:** REQUIRED SUB-SKILL: superpowers:executing-plans; parent executes inline, one final independent review.

**Goal:** Implement interplane.io#6 on existing site7 source without deployment or another factual collector.
**Architecture:** One explicit six-route registry drives generation, navigation, metadata, llms and sitemap. Reuse Facts, existing page sections, Maud and discovery verification. Homepage keeps its design; five doc pages reuse stylesheet and pinned source links.
**Tech Stack:** Existing Rust/Maud/serde_json, no new dependencies or executable JS.
**Spec:** https://github.com/aien-dev/interplane.io/issues/6 (fresh body/thread read).

## Global constraints
- No nginx/Pi/cron/deployment changes; no model or GPU use.
- Current protocol pin b669d06e21c9debb15073289d7c0b78b1b2d362c, gathered through existing collector. Preserve its actual failure verdict and planned-stage labels.
- Overview /, Quick Start /quick-start/, Concepts /concepts/, Adapter /adapter/, Conformance /conformance/, Status /status/ only. No invented supported-adapter list or extra collection.
- Clone/download and Cargo fetch require network; the scripted example itself is offline after prerequisites. Exact pin and commands must be replayed before final public-source handoff.
- This branch supersedes pending site7 ff50a39 only after qualification; old review is not approval of new routes.

## Review focus
- Duplicate/wrong canonical, title, description or JSON-LD must fail on every route.
- Cross-page/fragment links, local CSS/image/facts and sitemap/llms routes must resolve; unsafe traversal remains refused.
- Wrong factual status or unsupported-stage claims must not be introduced by page copy.
- Static navigation, skip focus, heading order and mobile overflow must be checked for all six routes.
- The offline example's real output/exit, network prerequisites and source pin must match the instructions; no mock success.

### Task 1: Generated six-route documentation site
Files: new src/routes.rs; modify page.rs/discovery.rs/main.rs/style.css/README; raw receipts under evidence/ONBOARDING-L642.
Interfaces: Route carries path/title/description; ALL is sole route inventory; render_route(Facts,Route), per-route structured metadata and verify consume it.
- [ ] Add behavior tests that build all six route paths and prove unique title/canonical/description, navigation, pinned source and preserved failure verdict; watch missing-route RED.
- [ ] Implement registry, doc templates and shared navigation/footer/source record using existing sections and tokens.
- [ ] Extend crawler output and local-link/metadata checks to all routes, preserving all original homepage and negative checks. Add corruption and missing-route refusals.
- [ ] Run the exact pinned protocol offline exercise; write only actually observed outcome and explicit prerequisites into Quick Start.
- [ ] Run complete Cargo suite/build/verify; actual HTTP + desktop/mobile/keyboard smoke all routes, inspect rendered pages.
- [ ] Push source/evidence; independent whole-branch review, one test-backed important-fix pass. Route exact superseding candidate to sole Renda, leaving publication/issue closure open.
