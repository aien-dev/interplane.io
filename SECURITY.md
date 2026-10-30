# Security policy

## Reporting a vulnerability

Report privately. Do not open a public issue, pull request or discussion about it.

Report vulnerabilities privately through the repository's Security tab, "Report a vulnerability",
at https://github.com/aien-dev/interplane.io/security/advisories/new.

Include the commit or release you tested, the component (path), steps to reproduce, and what an
attacker gains. If the report involves a secret, say where it was exposed and do not send the
secret itself. Do not post proof-of-concept payloads publicly.

## Response targets

This project has one maintainer; the targets below are best effort and stated honestly.

- Acknowledgement: within 3 working days.
- Triage and severity: within 7 days of acknowledgement.
- Fix or mitigation for confirmed issues: aim for 30 days for high and critical, 90 days
  otherwise, with a status update at least every 14 days until closed.
- Disclosure: coordinated. An advisory is published with credit (unless you ask otherwise)
  once the fix is on the default branch. Default embargo 90 days, negotiable.

## Scope

This repository builds the static INTERPLANE website. One Rust program, `crates/site-builder`,
reads the protocol repository's public record on GitHub, writes the facts to `facts.json`, renders
`dist/index.html` with Maud templates, and refuses a page that breaks the site rules.
`scripts/sync-to-hub.sh` is the hourly deploy job.

In scope:

- Text from the protocol repository (README tables, descriptions, commit status contexts) that
  reaches the page unescaped, injects markup or script, or produces a figure the protocol
  repository does not record.
- A page that passes `site-builder verify` while breaking a rule AGENTS.md lists: static HTML and
  CSS only, no scripts, no outside fonts or trackers, no plain `http://` links, browser requests to
  interplane.io only, the Spark verdict shown exactly as read.
- `scripts/sync-to-hub.sh` publishing a dirty checkout, a page that failed `verify`, or anything
  outside the build output.
- Tokens or SSH material written to tracked files, `facts.json` or logs.

Out of scope:

- The protocol itself, which lives in its own repository; GitHub, the `gh` CLI, the web server
  that hosts the site, and the host operating system.

## Supported versions

Only the default branch `main`, which is what the hourly job publishes. Reports against older commits are welcome but fixes land on the default branch only.

## Status

The live site is built from `main` by the hourly job (AGENTS.md). No hosted CI workflow is tracked
at this revision. No certification, no bug bounty. See AUDIT_SCOPE.md for what to review and how to build and test it.
