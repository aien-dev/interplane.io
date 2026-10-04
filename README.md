# interplane.io

Source of the static website for [INTERPLANE](https://github.com/aien-dev/interplane): plain HTML and CSS,
no framework, no scripts, no tracking, no external fonts.

- `index.html`: the page
- `public/style.css`: light and dark styles

All figures on the page come from `aien-dev/interplane` at commit 9aff24a (STATUS.md, ROADMAP.md,
docs/REPORT-0.1.md, bench/PROTOCOL-0.2.md, bench/runs/qual-20261004T2207Z/summary.md, qualification reports).
Update them when those files change.

## Hosting: hub Pi

The site is served from the house server ("hub", a Raspberry Pi), not from GitHub Pages. This repository
holds the source only; it has no Pages workflow and no CNAME file. Deploying means copying `index.html`
and `public/` into the web root on the hub and serving it for `interplane.io` and `www.interplane.io`.
DNS for the domain (registered and hosted at Gandi) points at the hub; the orchestrator handles both.

## License

AGPL-3.0-or-later (see `LICENSE`).
