# interplane.io

Source of the website for [INTERPLANE](https://github.com/aien-dev/interplane), the open
interoperability protocol between AI models and agent runtimes.

Part of [Aienix](https://github.com/aien-dev/aienix), the core tree of AIEN. INTERPLANE is one of
its attachments: it lives in its own repository and plugs into the core at a boundary.

**Status (2026-10-18):** live at [interplane.io](https://interplane.io). The current Rust generator
landed on 2026-10-16 (pull request #4), and the page is rebuilt hourly.

The page is not written by hand. A small Rust program, `crates/site-builder`, reads the protocol
repository's public record on GitHub (STATUS.md, ROADMAP.md, the stage reports, the latest tag,
the open proposals and the Spark judge's verdict for the current commit of main), writes those
facts to `facts.json`, and renders the page from Rust source plus that file. Every figure on the
page names the commit it was read at and links to it. The editorial copy lives in
`crates/site-builder/src/page.rs`.

## Rules the page keeps

`site-builder verify` refuses a build that breaks any of these:

- no executable scripts, no outside fonts, no trackers; the only requests a visitor's browser makes are to interplane.io;
- one strictly parsed, HTML-safe `application/ld+json` block describes the website and exact protocol source pin; it runs no code;
- crawler files and every generated local target must match the build; `llms.txt` links all six generated routes and pinned protocol documentation;
- sitemap dates are omitted rather than using hourly fetch time as a content-change date;
- no em or en dashes;
- the pinned commit of the protocol repository is on the page;
- the Spark verdict is shown as it is (PASS, FAIL, BLOCKED, RUNNING or NO VERDICT), never summarised;
- no mention of the removed Python SDK and no "draft" footer.

## Build

```
cargo build --release -p site-builder
target/release/site-builder facts            # needs `gh` logged in; writes facts.json
target/release/site-builder build dist       # renders all six routes, copies public/, facts.json
target/release/site-builder verify dist
```

`cargo test -p site-builder` renders a sample page and runs the same rules against it.

## Developer onboarding candidate (issue #6)

The route inventory in `crates/site-builder/src/routes.rs` generates Overview (`/`),
Quick Start (`/quick-start/`), Concepts (`/concepts/`), Writing an Adapter (`/adapter/`),
Conformance (`/conformance/`) and Versioned Status (`/status/`). Each has a unique title,
description and canonical, inert metadata and static HTML navigation. The existing facts
collector and hosting scripts are unchanged; this source change does not deploy anything.

The Quick Start recipe has its own immutable protocol pin, explicitly separate from the
advancing source-status snapshot. Clone and Cargo fetch require network access; the scripted
exercise is offline after dependencies are fetched. It uses no real model or GPU.

For the literal rendered recipe replay and route corruption controls, after building dist:

```sh
python3 evidence/ONBOARDING-L642/qualify.py
```

This cold-clones the pinned source, so it needs network access and Git/Rust/Cargo. Raw receipts
and the source snapshot are in that evidence directory. The earlier `evidence/discoverability-issue7`
receipts describe their original pinned single-route source, not qualification of these new routes.

## Hosting and automation

The site is served by nginx on the project's Raspberry Pi house server. GitHub Pages is not used.
`scripts/sync-to-hub.sh` runs from cron on the Spark once an hour: it fast-forwards a clean clone
of this repository, rebuilds, refreshes the facts (keeping the last good facts if GitHub is
unreachable), verifies, and copies the result to the house server only when something changed. It skips a
run while the Spark's quiet flag is held (the flag that reserves the machine for chip runs).

## Files

- `crates/site-builder/`: the generator (facts, page, verify).
- `public/style.css`: the one stylesheet; light and dark follow the reader's system setting.
- `public/hero.webp`: the hero illustration.
- `scripts/sync-to-hub.sh`: the hourly deploy.

## License

AGPL-3.0-or-later (see `LICENSE`).

## Human-Directed Agentic Engineering

Development follows Human-Directed Agentic Engineering (HDAE): humans define goals,
architecture, constraints, acceptance criteria, and stopping points; agents implement,
execute checks, diagnose failures, and revise within those boundaries. Human review
retains responsibility for claims, acceptance, and release decisions.

For the documented method and a systems-software case study, see Gregory M. Kurtzer,
*Ginux: Building an Independent Kernel That Runs Linux Userspace through Human-Directed
Agentic Engineering* (2026), https://doi.org/10.5281/zenodo.23682548.
