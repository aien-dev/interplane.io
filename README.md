# interplane.io

Source of the static website for [INTERPLANE](https://github.com/aien-dev/interplane): one
self-contained HTML page, no framework, no analytics or tracking scripts.

- `index.html`: the whole page. Its styles are inline and its hero image is embedded as base64 WebP.
  One small inline script drives the results tabs, the token bars and the copy button for the clone
  command. The page loads the IBM Plex fonts from Google Fonts (`fonts.googleapis.com`,
  `fonts.gstatic.com`), so a visitor's browser contacts Google. That is its only external request.
  The footer still says "Draft marketing page ... not yet live", although interplane.io already
  serves this page.
- `public/style.css`: light and dark styles from the earlier version of the page. The current
  `index.html` does not link it.

All figures on the page come from `aien-dev/interplane` at commit 9aff24a (STATUS.md, ROADMAP.md,
docs/REPORT-0.1.md, bench/PROTOCOL-0.2.md, bench/runs/qual-20261004T2207Z/summary.md, qualification reports).
Update them when those files change. They are a snapshot: `aien-dev/interplane` has moved on since
(ROADMAP.md there records 0.2 and 0.3 as closed, while the page's roadmap section still shows 0.2 in
progress and 0.3 specified).

## Hosting: hub Pi

The site is served from the house server ("hub", a Raspberry Pi), not from GitHub Pages. This repository
holds the source only; it has no Pages workflow and no CNAME file. Deploying means copying `index.html`
and `public/` into the web root on the hub and serving it for `interplane.io` and `www.interplane.io`.
DNS for the domain (registered and hosted at Gandi) points at the hub; the orchestrator handles both.

## License

AGPL-3.0-or-later (see `LICENSE`).
