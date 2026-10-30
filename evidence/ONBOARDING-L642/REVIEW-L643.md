# L643 independent review and disposition

Reviewed source: `fcb834efeab2e323dfebf1f9ee761d743770e171` relative to
`ff50a39c79fb8b52390242a51c6b87a7bf59ab6f`.
Reviewer: `deleg_e3224361`, `sa-0-46da7fae`, read-only independent whole-branch review.

## Actual returned finding

**P2:** `discovery.rs` skipped every HTTPS URL in both href and src. A built
route with `<img src="https://tracker.example/pixel">` still verified with exit 0,
contradicting the stated same-origin resource rule. External navigation is valid;
automatically loaded network resources are not.

The reviewer independently ran the complete workspace suite (13/13), built and
verified all six pages, refused 18 wrong/duplicate title/description mutations,
and verified the saved 72-control and literal Quick Start raw hashes. Deployment
and publication were not judged. No other findings were reported.

## Parent disposition: fixed, not waived

Treat this P2 as an Important acceptance defect because it violates an explicit
published resource policy. Parent reproduced it with
`remote_src_is_refused_but_external_links_remain_valid`: RED exit 101, reporting
`remote resource accepted on /` before the fix.

Correction source: `87d140195ddcb491fc813ffe951a7a5ce86fce54`.
The verifier now distinguishes external href navigation from network-loaded src.
Generated resources must use checked local paths (embedded data remains valid).
The regression exercises every route: HTTPS tracking image refused, external
GitHub navigation and a real local image accepted. This is a generated-output
invariant for the generator's quoted attributes, not a general HTML sanitizer.

After correction, complete workspace suite **14/14 passed**. Native release
build and verify passed. All **12 generated files** remained byte-identical to
the earlier desktop/mobile-qualified output; the fix changes verification, not
page content. The complete 72-control/literal cold recipe qualification was also
repeated on the corrected source; it passed with the same actual four decision
rows and final OK line. See [post-review qualification](post-review/qualification.json),
[RED log](post-review/remote-src-red.log.gz), and [14-test suite](post-review/post-review-tests.log.gz).

No second independent review is claimed; the one fix pass is verified by the
new RED/GREEN regression and green full suite. Original evidence at
`ec5af4caa8a7b8206f818e62f32b02c80e297b7f` remains unchanged and is historical,
not falsely relabelled as testing the correction.

## Remaining delivery boundary

Source is locally qualified for Renda's exact-head publication/integration gates,
not merged, deployed or issue-closed. The new source includes and supersedes the
pending site7 discoverability candidate; it must not be independently merged
alongside an older overlapping READY branch. Existing repository identity policy
and the sole Renda publication owner remain authoritative.
