//! Facts read from the protocol repository's public record on GitHub. `site-builder facts`
//! writes them to `facts.json`; `site-builder build` renders them. Nothing in this file is
//! typed by hand: every value names the commit it was read at, so the page can link to it.

use std::path::Path;
use std::process::Command;

use serde::{Deserialize, Serialize};

/// The protocol repository under its current name. The old name `aien-dev/interplane` still
/// redirects, but a redirect stops working if that name is ever reused, so ask for the real one.
pub const PROTOCOL_REPO: &str = "aien-dev/interplane-mv";

/// The commit status context under which the Spark judge posts its verdict.
pub const SPARK_STATUS_CONTEXT: &str = "AIEN Verification (Spark)";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Pin {
    /// The repository as GitHub names it today.
    pub full_name: String,
    pub url: String,
    /// The commit of main every fact below was read at.
    pub sha: String,
    pub committed_at: String,
    pub commit_title: String,
    pub latest_tag: String,
    pub license: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct StatusCounts {
    pub rows: usize,
    pub qualified: usize,
    pub conformant: usize,
    pub tested: usize,
    pub implemented: usize,
    pub specified: usize,
    pub other: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Conformance {
    pub passed: u32,
    pub total: u32,
    pub note: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Stage {
    pub version: String,
    pub layer: String,
    pub goal: String,
    /// `closed`, `next`, `reported`, or `planned`.
    pub state: String,
    /// `PASS`, `FAIL`, or empty when no gate verdict is recorded.
    pub verdict: String,
    pub closed_on: String,
    /// The report for this stage, when one exists.
    pub report_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Report {
    pub name: String,
    pub title: String,
    pub headline: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Merge {
    pub number: u64,
    pub title: String,
    pub merged_at: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Receipt {
    /// `success`, `failure`, `pending`, or `none`.
    pub state: String,
    pub description: String,
    pub target_url: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Facts {
    pub generated_at: String,
    pub pin: Pin,
    pub status: StatusCounts,
    pub conformance: Option<Conformance>,
    pub stages: Vec<Stage>,
    pub next_stage: String,
    pub reports: Vec<Report>,
    pub open_issues: u64,
    pub open_prs: u64,
    pub last_merge: Option<Merge>,
    pub receipt: Receipt,
}

impl Facts {
    pub fn load(path: &Path) -> Result<Facts, String> {
        let text = std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
        serde_json::from_str(&text).map_err(|e| format!("{} is not a facts file: {e}", path.display()))
    }

    pub fn save(&self, path: &Path) -> Result<(), String> {
        let text = serde_json::to_string_pretty(self).map_err(|e| e.to_string())?;
        std::fs::write(path, text + "\n").map_err(|e| format!("{}: {e}", path.display()))
    }

    /// A link into the pinned commit of the protocol repository.
    pub fn blob(&self, path: &str) -> String {
        format!("{}/blob/{}/{}", self.pin.url, self.pin.sha, path)
    }
}

fn gh(args: &[&str]) -> Result<serde_json::Value, String> {
    let out = Command::new("gh").arg("api").args(args).output().map_err(|e| format!("gh api: {e}"))?;
    if !out.status.success() {
        return Err(format!("gh api {}: {}", args.join(" "), String::from_utf8_lossy(&out.stderr).trim()));
    }
    serde_json::from_slice(&out.stdout).map_err(|e| format!("gh api {}: bad JSON: {e}", args.join(" ")))
}

fn gh_text(full: &str, path: &str, sha: &str) -> Result<String, String> {
    let out = Command::new("gh")
        .args(["api", "-H", "Accept: application/vnd.github.raw+json", &format!("repos/{full}/contents/{path}?ref={sha}")])
        .output()
        .map_err(|e| format!("gh api: {e}"))?;
    if !out.status.success() {
        return Err(format!("gh api {path}@{sha}: {}", String::from_utf8_lossy(&out.stderr).trim()));
    }
    Ok(String::from_utf8_lossy(&out.stdout).into_owned())
}

fn s(v: &serde_json::Value, key: &str) -> String {
    v.get(key).and_then(|x| x.as_str()).unwrap_or("").to_string()
}

fn ptr(v: &serde_json::Value, path: &str) -> String {
    v.pointer(path).and_then(|x| x.as_str()).unwrap_or("").to_string()
}

pub fn gather() -> Result<Facts, String> {
    let repo = gh(&[format!("repos/{PROTOCOL_REPO}").as_str()])?;
    let full = s(&repo, "full_name");
    let url = s(&repo, "html_url");
    let commit = gh(&[format!("repos/{full}/commits/main").as_str()])?;
    let sha = s(&commit, "sha");
    let tags = gh(&[format!("repos/{full}/tags?per_page=1").as_str()])?;
    let pin = Pin {
        full_name: full.clone(),
        url: url.clone(),
        sha: sha.clone(),
        committed_at: ptr(&commit, "/commit/committer/date"),
        commit_title: ptr(&commit, "/commit/message").lines().next().unwrap_or("").to_string(),
        latest_tag: tags.get(0).map(|t| s(t, "name")).unwrap_or_default(),
        license: ptr(&repo, "/license/spdx_id"),
        description: s(&repo, "description"),
    };

    let status_md = gh_text(&full, "STATUS.md", &sha)?;
    let status = status_counts(&status_md);
    let conformance = conformance_row(&status_md);

    let roadmap_md = gh_text(&full, "ROADMAP.md", &sha)?;
    let docs = gh(&[format!("repos/{full}/contents/docs?ref={sha}").as_str()])?;
    let report_names: Vec<String> = docs
        .as_array()
        .map(|a| a.iter().map(|d| s(d, "name")).filter(|n| n.starts_with("REPORT-") && n.ends_with(".md")).collect())
        .unwrap_or_default();
    let mut reports = Vec::new();
    for name in &report_names {
        let text = gh_text(&full, &format!("docs/{name}"), &sha)?;
        reports.push(Report {
            name: name.clone(),
            title: text.lines().find(|l| l.starts_with("# ")).map(|l| l[2..].trim().to_string()).unwrap_or_else(|| name.clone()),
            headline: headline(&text),
            url: format!("{url}/blob/{sha}/docs/{name}"),
        });
    }
    let (stages, next_stage) = stages(&roadmap_md, &reports);

    let pulls_open = gh(&[format!("repos/{full}/pulls?state=open&per_page=100").as_str()])?;
    let open_prs = pulls_open.as_array().map(|a| a.len() as u64).unwrap_or(0);
    let open_issues = repo.get("open_issues_count").and_then(|n| n.as_u64()).unwrap_or(0).saturating_sub(open_prs);
    let pulls_closed = gh(&[format!("repos/{full}/pulls?state=closed&sort=updated&direction=desc&per_page=30").as_str()])?;
    let last_merge = pulls_closed.as_array().and_then(|a| {
        a.iter().filter(|p| !s(p, "merged_at").is_empty()).max_by(|a, b| s(a, "merged_at").cmp(&s(b, "merged_at"))).map(|p| Merge {
            number: p.get("number").and_then(|n| n.as_u64()).unwrap_or(0),
            title: s(p, "title"),
            merged_at: s(p, "merged_at"),
            url: s(p, "html_url"),
        })
    });

    // The Spark judge's verdict for this commit, never the hosted bridge check run.
    let combined = gh(&[format!("repos/{full}/commits/{sha}/status").as_str()])?;
    let receipt = combined
        .get("statuses")
        .and_then(|a| a.as_array())
        .and_then(|a| a.iter().filter(|r| s(r, "context") == SPARK_STATUS_CONTEXT).max_by(|a, b| s(a, "updated_at").cmp(&s(b, "updated_at"))))
        .map(|st| Receipt {
            state: match s(st, "state").as_str() {
                "success" => "success".to_string(),
                "failure" | "error" => "failure".to_string(),
                _ => "pending".to_string(),
            },
            description: public_description(&s(st, "description")),
            target_url: {
                let t = s(st, "target_url");
                if t.is_empty() { format!("{url}/commit/{sha}") } else { t }
            },
            updated_at: s(st, "updated_at"),
        })
        .unwrap_or(Receipt {
            state: "none".to_string(),
            description: "The Spark has not published a verdict for this commit yet.".to_string(),
            target_url: String::new(),
            updated_at: String::new(),
        });

    Ok(Facts {
        generated_at: now_utc(),
        pin,
        status,
        conformance,
        stages,
        next_stage,
        reports,
        open_issues,
        open_prs,
        last_merge,
        receipt,
    })
}

/// Splits a Markdown table row into trimmed cells.
fn cells(line: &str) -> Vec<String> {
    let t = line.trim();
    if !t.starts_with('|') {
        return Vec::new();
    }
    t.trim_matches('|').split('|').map(|c| c.trim().to_string()).collect()
}

fn is_separator(row: &[String]) -> bool {
    !row.is_empty() && row.iter().all(|c| !c.is_empty() && c.chars().all(|ch| ch == '-' || ch == ':'))
}

/// Counts STATUS.md rows by the strongest status word in the Rust column.
pub fn status_counts(md: &str) -> StatusCounts {
    let mut c = StatusCounts { rows: 0, qualified: 0, conformant: 0, tested: 0, implemented: 0, specified: 0, other: 0 };
    let mut in_table = false;
    for line in md.lines() {
        let row = cells(line);
        if row.len() < 2 {
            in_table = false;
            continue;
        }
        if row[0] == "Component" {
            in_table = true;
            continue;
        }
        if !in_table || is_separator(&row) {
            continue;
        }
        c.rows += 1;
        let rust = row[1].to_lowercase();
        if rust.contains("qualified") {
            c.qualified += 1;
        } else if rust.contains("conformant") {
            c.conformant += 1;
        } else if rust.contains("tested") {
            c.tested += 1;
        } else if rust.contains("implemented") || rust.contains("running") {
            c.implemented += 1;
        } else if rust.contains("specified") {
            c.specified += 1;
        } else {
            c.other += 1;
        }
    }
    c
}

/// The "Conformance runner" row: `passed/total` from its fixtures column.
pub fn conformance_row(md: &str) -> Option<Conformance> {
    for line in md.lines() {
        let row = cells(line);
        if row.len() >= 4 && row[0].starts_with("Conformance runner") {
            let fx = &row[3];
            let (a, b) = fx.split_once('/')?;
            let passed: u32 = a.trim().parse().ok()?;
            let total: u32 = b.trim().chars().take_while(|c| c.is_ascii_digit()).collect::<String>().parse().ok()?;
            return Some(Conformance { passed, total, note: fx.clone() });
        }
    }
    None
}

/// The first "**Headline: ...**" line of a report, without the Markdown bold marks.
pub fn headline(md: &str) -> String {
    let mut out = String::new();
    let mut collecting = false;
    for line in md.lines() {
        if !collecting {
            if let Some(rest) = line.trim_start().strip_prefix("**Headline:") {
                collecting = true;
                out.push_str(rest.trim());
            }
            continue;
        }
        if line.trim().is_empty() {
            break;
        }
        out.push(' ');
        out.push_str(line.trim());
    }
    // Keep the headline sentence only; the rest of the paragraph is detail.
    let one = out.replace("**", "");
    let one = one.trim().to_string();
    match one.find(". ") {
        Some(i) => one[..=i].to_string(),
        None => one,
    }
}

/// Roadmap stages from the version table, with their state from the "closed" headings and
/// the last "Next:" line.
pub fn stages(md: &str, reports: &[Report]) -> (Vec<Stage>, String) {
    let mut stages: Vec<Stage> = Vec::new();
    let mut in_table = false;
    for line in md.lines() {
        let row = cells(line);
        if row.len() < 4 {
            in_table = false;
            continue;
        }
        if row[0] == "Version" {
            in_table = true;
            continue;
        }
        if !in_table || is_separator(&row) {
            continue;
        }
        stages.push(Stage {
            version: row[0].clone(),
            layer: row[1].clone(),
            goal: row[3].clone(),
            state: "planned".to_string(),
            verdict: String::new(),
            closed_on: String::new(),
            report_url: String::new(),
        });
    }
    let mut next = String::new();
    for line in md.lines() {
        let t = line.trim();
        if let Some(rest) = t.strip_prefix("## ") {
            // e.g. "0.3 closed (2026-10-05): gate PASS"
            let mut words = rest.split_whitespace();
            let v = words.next().unwrap_or("");
            if words.next() == Some("closed") {
                if let Some(st) = stages.iter_mut().find(|s| s.version == v) {
                    st.state = "closed".to_string();
                    st.closed_on = rest.find('(').and_then(|a| rest[a + 1..].find(')').map(|b| rest[a + 1..a + 1 + b].to_string())).unwrap_or_default();
                    let upper = rest.to_uppercase();
                    st.verdict = if upper.contains("PASS") {
                        "PASS".to_string()
                    } else if upper.contains("FAIL") {
                        "FAIL".to_string()
                    } else {
                        String::new()
                    };
                }
            }
        }
        if let Some(rest) = t.strip_prefix("Next: **") {
            next = rest.split("**").next().unwrap_or("").trim_end_matches('.').to_string();
        }
    }
    for st in stages.iter_mut() {
        if let Some(r) = reports.iter().find(|r| r.name == format!("REPORT-{}.md", st.version)) {
            st.report_url = r.url.clone();
            if st.state == "planned" {
                st.state = "reported".to_string();
            }
        }
        if !next.is_empty() && next.starts_with(&st.version) && st.state != "closed" {
            st.state = "next".to_string();
        }
    }
    (stages, next)
}

/// The Spark writes its verdict line followed by the bundle path on its own disk. The verdict
/// is public; the path is not useful to anyone else, so it is cut off here.
pub fn public_description(raw: &str) -> String {
    let cut = raw.find(": bundle ").or_else(|| raw.find(" bundle /")).unwrap_or(raw.len());
    raw[..cut].trim().to_string()
}

pub fn now_utc() -> String {
    let out = Command::new("date").args(["-u", "+%Y-%m-%dT%H:%M:%SZ"]).output();
    match out {
        Ok(o) if o.status.success() => String::from_utf8_lossy(&o.stdout).trim().to_string(),
        _ => "unknown".to_string(),
    }
}

/// `2026-10-16T09:30:35Z` to `16 Oct 2026`. Anything else is returned unchanged.
pub fn short_date(ts: &str) -> String {
    const MONTHS: [&str; 12] = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
    let date = ts.split('T').next().unwrap_or(ts);
    let parts: Vec<&str> = date.split('-').collect();
    if parts.len() != 3 {
        return ts.to_string();
    }
    let (y, m, d) = (parts[0], parts[1].parse::<usize>().unwrap_or(0), parts[2].parse::<u32>().unwrap_or(0));
    if !(1..=12).contains(&m) || d == 0 {
        return ts.to_string();
    }
    format!("{d} {} {y}", MONTHS[m - 1])
}

#[cfg(test)]
mod tests {
    use super::*;

    const STATUS: &str = "# STATUS\n\n| Component | Rust | Python | Fixtures | Notes |\n|---|---|---|---|---|\n| Core schemas | specified | specified | validated | x |\n| Lenshift qwen35 | conformant, qualified | conformant | 10 | |\n| Probe | qualified | qualified | - | |\n| Adapter | tested | - | 3 | |\n| Conformance runner | implemented | implemented | 106/106 (verdicts identical) | `--dump` |\n| Reproducibility pins | - | - | - | see docs |\n\nAfter.\n";

    const ROADMAP: &str = "# roadmap\n\n| Version | Layer | Builds | Goal |\n|---|---|---|---|\n| 0.1 | Reality | a | Prove it |\n| 0.2 | Capability | b | Minimum tools |\n| 0.3 | Trust | c | Keep the line |\n| 0.4 | Execution | d | Multi-round |\n| 0.5 | Qualification | e | Measured claims |\n\n## 0.2 closed (2026-10-04): gate FAIL recorded\n\ntext\n\n## 0.3 closed (2026-10-05): gate PASS\n\ntext\n\nNext: **0.4 Execution**.\n";

    #[test]
    fn counts_status_rows_by_strongest_word() {
        let c = status_counts(STATUS);
        assert_eq!(c.rows, 6);
        assert_eq!(c.qualified, 2);
        assert_eq!(c.conformant, 0);
        assert_eq!(c.tested, 1);
        assert_eq!(c.implemented, 1);
        assert_eq!(c.specified, 1);
        assert_eq!(c.other, 1);
    }

    #[test]
    fn reads_the_conformance_row() {
        let c = conformance_row(STATUS).unwrap();
        assert_eq!((c.passed, c.total), (106, 106));
    }

    #[test]
    fn reads_stage_states_from_headings() {
        let reports = vec![Report { name: "REPORT-0.1.md".into(), title: "r".into(), headline: "h".into(), url: "u".into() }];
        let (st, next) = stages(ROADMAP, &reports);
        assert_eq!(next, "0.4 Execution");
        let by = |v: &str| st.iter().find(|s| s.version == v).unwrap().clone();
        assert_eq!(by("0.1").state, "reported");
        assert_eq!(by("0.2").state, "closed");
        assert_eq!(by("0.2").verdict, "FAIL");
        assert_eq!(by("0.3").verdict, "PASS");
        assert_eq!(by("0.3").closed_on, "2026-10-05");
        assert_eq!(by("0.4").state, "next");
        assert_eq!(by("0.5").state, "planned");
    }

    #[test]
    fn headline_keeps_the_first_sentence_without_bold() {
        let md = "# r\n\n**Headline: every offline gate of the pre-registered 0.3 protocol PASSED** (P, X;\nthresholds fixed). More.\n\n## Observed\n";
        assert_eq!(headline(md), "every offline gate of the pre-registered 0.3 protocol PASSED (P, X; thresholds fixed).");
        assert_eq!(headline("no headline here"), "");
    }

    #[test]
    fn public_description_drops_the_bundle_path() {
        assert_eq!(public_description("PASS 12 gates: bundle /home/x/y"), "PASS 12 gates");
        assert_eq!(public_description("PASS"), "PASS");
    }

    #[test]
    fn short_date_reads_rfc3339() {
        assert_eq!(short_date("2026-10-16T09:30:35Z"), "16 Oct 2026");
        assert_eq!(short_date("nope"), "nope");
    }
}
