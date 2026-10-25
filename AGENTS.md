# interplane.io (source of the INTERPLANE website)

> The static website for INTERPLANE, the open interoperability protocol between AI models and agent runtimes. The page is not written by hand: one Rust program, `crates/site-builder`, reads the protocol repository's public record on GitHub (aien-dev/interplane-mv), writes those facts to `facts.json`, renders `dist/index.html` from Maud templates plus that file, and refuses a build that breaks the page's rules.

Shared rules: read https://www.aienos.com/agents/RULES.md first. This file adds what is specific to this repository; where the two differ, this file wins here.

## Scope

- Applies to the whole repository.
- `crates/site-builder/src/`: the generator. `facts.rs` reads the protocol repository, `page.rs` holds the editorial copy and the markup, `main.rs` has the `facts`, `build` and `verify` commands and the tests.
- `public/`: `style.css` (the one stylesheet) and `hero.webp`, copied into the output.
- `scripts/sync-to-hub.sh`: the hourly deploy job.
- `dist/` and `facts.json` are generated and ignored by git. Never edit or commit them.
- The protocol itself, its numbers and its status live in aien-dev/interplane-mv. Change them there, not here.

## Installation

```bash
git clone https://github.com/aien-dev/interplane.io.git
cd interplane.io
cargo build --release -p site-builder
```

You need a stable Rust toolchain; the crate depends on `maud`, `serde` and `serde_json`. The `facts` step also needs the GitHub CLI `gh`, logged in, with network access to GitHub. There is no Node build and no package manager besides Cargo.

## Configuration

- `facts.json` (generated) holds every figure the page shows; `build` reads it from the path you give (default `facts.json`).
- The protocol repository slug and the Spark status context are constants at the top of `crates/site-builder/src/facts.rs` (`PROTOCOL_REPO`, `SPARK_STATUS_CONTEXT`).
- `scripts/sync-to-hub.sh` reads `REPO`, `HUB`, `WEBDIR` and `LOGDIR` from the environment; its header documents each one.

## Usage

```bash
# Tests: render a sample page and run the verify rules against it
cargo test -p site-builder

# Refresh the facts from GitHub (needs gh logged in and network)
target/release/site-builder facts            # writes facts.json

# Render and check the page
target/release/site-builder build dist       # dist/index.html, public/, facts.json
target/release/site-builder verify dist
```

`scripts/sync-to-hub.sh` is the production deploy. It runs from cron on the DGX Spark, rebuilds from a clean clone of `main`, keeps the last good facts if GitHub is unreachable, refuses a page that fails `verify`, and copies the result over SSH to the nginx server on the project's Raspberry Pi. Do not run it yourself; it needs that host and its SSH access.

## Conventions

- Every figure on the page comes from the protocol repository and names the commit it was read at. Leave out anything that repository does not record; never type a number into `page.rs`.
- Static HTML and CSS only: no scripts, no outside fonts, no trackers, no plain `http://` links. The only requests a visitor's browser makes are to interplane.io.
- No em dashes or en dashes in the page or the stylesheet.
- The Spark verdict is shown exactly as read (PASS, FAIL, BLOCKED, RUNNING or NO VERDICT), never summarised.
- No mention of the removed Python SDK, no "draft" footer, and no outside projects named as users or partners.
- `site-builder verify` enforces these rules. When you add a rule, add the check to `verify` and keep `cargo test -p site-builder` passing.

## Safety and approvals

- The site is served from the project's Raspberry Pi, not GitHub Pages. Do not add a Pages workflow or a `CNAME` file.
- Ask before changing the deploy script, the hosting, or a claim the page makes about the protocol.
- Never commit `facts.json`, `dist/`, `.crumb.local`, tokens or SSH material. `facts` uses the `gh` login and never writes a token.
- Work on a branch and merge through a pull request; `main` is what the hourly job publishes.

## Further reading

- `README.md`: the page's rules, the build steps and the hosting.
- https://interplane.io: the live site.
- https://github.com/aien-dev/interplane-mv: the protocol repository the page reads.
