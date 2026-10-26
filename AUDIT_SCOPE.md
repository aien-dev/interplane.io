# Audit scope

Written for an outside, independent security audit. Revision described:
`62d5b27beea12c8fb75d266ffc456304af1768ae` on `main`, 2026-10-26. Everything below is taken from the repository's own files at
that revision; items marked NOT_RUN were not executed while writing this document.

## What this repository is

The source of the INTERPLANE website. A Cargo workspace with one crate, `crates/site-builder`
(version 0.2.0, `publish = false`): `facts.rs` (488 lines) reads the protocol repository through
`gh api`, `page.rs` (304) holds the copy and the Maud markup, `main.rs` (221) has the `facts`,
`build` and `verify` commands and tests. `public/` holds `style.css` and `hero.webp`.
`scripts/sync-to-hub.sh` (71 lines) is the deploy job. `dist/` and `facts.json` are generated and
ignored. Licence AGPL-3.0-or-later.

## What to review, in priority order

1. `crates/site-builder/src/page.rs`: every place a fact string is placed into markup. Maud
   escapes `(expr)` by default; no `PreEscaped` appears in the source at this revision. Check
   attribute values and link targets built from facts (for example URLs that could become
   `javascript:` or `http://`).
2. `verify_html` and `verify` in `main.rs`: the rule checks that gate publication. Check each
   AGENTS.md rule is enforced and cannot be bypassed by unusual casing or encoding.
3. `crates/site-builder/src/facts.rs`: `gh` and `gh_text` call `gh api` with arguments built from
   the constant `PROTOCOL_REPO` and values read from earlier responses (commit sha, file paths);
   Markdown parsers `status_counts`, `conformance_row`, `headline`, `stages`,
   `public_description`; status selection by `SPARK_STATUS_CONTEXT`. Note: AGENTS.md names the
   protocol repository `aien-dev/interplane-mv`, while `PROTOCOL_REPO` is `aien-dev/interplane`;
   which one GitHub resolves is unknown from the repository.
4. `scripts/sync-to-hub.sh`: skips while the machine's quiet flag is held, clones or fast-forwards
   `main`, refuses a dirty checkout, builds with `cargo build -q --release -p site-builder` (no
   `--locked`), falls back to the last good `facts.json` when `facts` fails, refuses on `verify`
   failure, compares a `cksum` with the last published one, then `rsync -a --delete` to the web
   host over SSH. `REPO`, `HUB`, `WEBDIR`, `LOGDIR` come from the environment.
5. `copy_dir` and `build` in `main.rs`: copying `public/` into the output.

## Trust boundaries

- Untrusted input: the protocol repository's README and status data as returned by the GitHub
  API. Anyone who can merge there influences the page text.
- Trusted: the `gh` login used by `facts` (AGENTS.md: it never writes a token), the deploy host's
  SSH access to the web server, and `main` of this repository (merges publish within an hour).
- Visitor side: the output is static HTML and CSS.

## Threat model summary

- Assets: integrity of the public site; honesty of the figures it shows; the deploy SSH key.
- Attackers considered: hostile or mistaken text in the protocol repository; an unreviewed change
  on `main`.
- Defended: Maud escaping, `verify` rules before publication, refusal of dirty checkouts,
  last-good-facts fallback.
- Not defended / explicitly out of scope: the web server configuration, TLS for interplane.io and
  the deploy host are outside the repository; `rsync --delete` mirrors whatever `build` produced.

## Build from source at a pin

Needs a stable Rust toolchain (version not pinned). Dependencies `maud` 0.26, `serde` 1,
`serde_json` 1; `Cargo.lock` committed (18 packages).

```sh
cargo build --release -p site-builder
```

`site-builder facts` needs `gh`, logged in, and network access to GitHub.

## Run every test

NOT_RUN while writing this document. From AGENTS.md:

```sh
cargo test -p site-builder                       # renders a sample page and runs the verify rules
target/release/site-builder facts                # needs gh login and network; writes facts.json
target/release/site-builder build dist
target/release/site-builder verify dist
```

Test attributes at this revision: 6 in `facts.rs`, 3 in `main.rs`. Expected: `test result: ok`.
`scripts/sync-to-hub.sh`: do not run (AGENTS.md); it needs the deploy host and its SSH access.
NOT_RUN.

## Known limitations and NOT_RUN

- No tracked CI workflow at this revision.
- The repository name mismatch in item 3 above.
- The deploy build does not pass `--locked`.
- Hosting, TLS and web server configuration are not in this repository.

## Dependencies and provenance

- `Cargo.lock` committed; crates from crates.io.
- Build-time tools for the deploy: `git`, `cargo`, `gh`, `rsync`, `ssh`, `cksum`; versions not
  pinned.
- AGENTS.md: no Node build, no package manager besides Cargo; no GitHub Pages workflow or `CNAME`.

## Evidence

No receipts are stored in this repository. Every figure on the page names the protocol commit it
was read at (AGENTS.md). The deploy job keeps its log and last facts outside the repository.
