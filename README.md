# interplane.io

Source of the website for [INTERPLANE](https://github.com/aien-dev/interplane), the open
interoperability protocol between AI models and agent runtimes.

The page is not written by hand. A small Rust program, `crates/site-builder`, reads the protocol
repository's public record on GitHub (STATUS.md, ROADMAP.md, the stage reports, the latest tag,
the open proposals and the Spark judge's verdict for the current commit of main), writes those
facts to `facts.json`, and renders the page from Rust source plus that file. Every figure on the
page names the commit it was read at and links to it. The editorial copy lives in
`crates/site-builder/src/page.rs`.

## Rules the page keeps

`site-builder verify` refuses a build that breaks any of these:

- no scripts, no outside fonts, no trackers; the only requests a visitor's browser makes are to interplane.io;
- no em or en dashes;
- the pinned commit of the protocol repository is on the page;
- the Spark verdict is shown as it is (PASS, FAIL, RUNNING or NO VERDICT), never summarised;
- no mention of the removed Python SDK and no "draft" footer.

## Build

```
cargo build --release -p site-builder
target/release/site-builder facts            # needs `gh` logged in; writes facts.json
target/release/site-builder build dist       # renders dist/index.html, copies public/, facts.json
target/release/site-builder verify dist
```

`cargo test -p site-builder` renders a sample page and runs the same rules against it.

## Hosting and automation

The site is served by the house server ("hub", a Raspberry Pi) behind nginx, not by GitHub Pages.
`scripts/sync-to-hub.sh` runs from cron on the Spark once an hour: it fast-forwards a clean clone
of this repository, rebuilds, refreshes the facts (keeping the last good facts if GitHub is
unreachable), verifies, and copies the result to the hub only when something changed. It skips a
run while the Spark's quiet flag is held. Its log is under `~/.local/share/interplane-site/`.

## Files

- `crates/site-builder/`: the generator (facts, page, verify).
- `public/style.css`: the one stylesheet; light and dark follow the reader's system setting.
- `public/hero.webp`: the hero illustration.
- `scripts/sync-to-hub.sh`: the hourly deploy.

## License

AGPL-3.0-or-later (see `LICENSE`).
