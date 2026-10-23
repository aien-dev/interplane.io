//! interplane.io site builder.
//!
//! ```text
//! site-builder facts  [facts.json]          read the protocol repository, write the facts file
//! site-builder build  [dist] [facts.json]   render the page and copy public/ into dist
//! site-builder verify [dist]                refuse a page that breaks the site rules
//! ```
//!
//! The page is rendered from Rust source plus the facts file. No executable scripts, no outside fonts, no
//! trackers, no em or en dashes, and no figure that does not resolve to a record in the
//! protocol repository.

mod discovery;
mod facts;
mod page;

use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use facts::Facts;

fn copy_dir(src: &Path, dst: &Path) -> std::io::Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let to = dst.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            copy_dir(&entry.path(), &to)?;
        } else {
            fs::copy(entry.path(), &to)?;
        }
    }
    Ok(())
}

fn build(dist: &Path, facts_path: &Path) -> Result<(), String> {
    let facts = Facts::load(facts_path)?;
    fs::create_dir_all(dist).map_err(|e| format!("{}: {e}", dist.display()))?;
    let html = page::render(&facts);
    fs::write(dist.join("index.html"), &html).map_err(|e| e.to_string())?;
    copy_dir(Path::new("public"), &dist.join("public")).map_err(|e| format!("public/: {e}"))?;
    fs::copy(facts_path, dist.join("facts.json")).map_err(|e| e.to_string())?;
    discovery::write(dist, &facts)?;
    println!("built {} ({} bytes) from {}@{}", dist.join("index.html").display(), html.len(), facts.pin.full_name, &facts.pin.sha[..12.min(facts.pin.sha.len())]);
    Ok(())
}

/// The site rules, enforced on the rendered output so a copy change cannot slip past them.
pub fn verify_html(html: &str, css: &str) -> Vec<String> {
    let mut problems = Vec::new();
    let mut need = |cond: bool, msg: &str| {
        if !cond {
            problems.push(msg.to_string());
        }
    };
    need(!html.contains('\u{2014}'), "em dash in index.html");
    need(!html.contains('\u{2013}'), "en dash in index.html");
    need(!css.contains('\u{2014}') && !css.contains('\u{2013}'), "dash in style.css");
    need(discovery::json_block(html).is_ok(), "invalid or executable script in index.html");
    need(!html.contains("fonts.googleapis") && !html.contains("fonts.gstatic") && !css.contains("@import") && !css.contains("fonts.g"), "an outside font");
    need(!html.contains("href=\"http://"), "a plain http link");
    need(!html.contains("Draft marketing page"), "the old draft footer is on the page");
    need(!html.contains("not yet live"), "the page calls itself not live");
    need(!html.contains("Python SDK"), "the removed Python SDK is mentioned");
    need(html.contains("id=\"status\""), "missing status section");
    need(html.contains("id=\"receipt\""), "missing receipt section");
    need(html.contains("id=\"modules\""), "missing modules section");
    need(html.contains("public/hero.webp"), "missing hero image");
    need(html.contains("href=\"facts.json\""), "missing facts.json link");
    need(html.contains("<title>INTERPLANE: "), "missing title");
    need(html.contains("og:title"), "missing og:title");
    for host in ["googletagmanager", "google-analytics", "plausible", "cloudflareinsights", "umami"] {
        need(!html.contains(host), &format!("tracker host {host} in index.html"));
    }
    let mut depth: i64 = 0;
    for ch in css.chars() {
        match ch {
            '{' => depth += 1,
            '}' => depth -= 1,
            _ => {}
        }
        if depth < 0 {
            break;
        }
    }
    need(depth == 0, "style.css braces do not balance");
    problems
}

fn verify(dist: &Path) -> Result<(), String> {
    let html = fs::read_to_string(dist.join("index.html")).map_err(|e| format!("index.html: {e}"))?;
    let css = fs::read_to_string(dist.join("public/style.css")).map_err(|e| format!("public/style.css: {e}"))?;
    let facts = Facts::load(&dist.join("facts.json"))?;
    let mut problems = verify_html(&html, &css);
    problems.extend(discovery::verify(dist, &html, &facts));
    if !dist.join("public/hero.webp").exists() {
        problems.push("public/hero.webp missing from dist".to_string());
    }
    // The hand-copied receipt figures must agree with the STATUS row they come from.
    let status_md_hint = format!("{} -> {} tools, {}% fewer", page::RECEIPT_TOOLS_BEFORE, page::RECEIPT_TOOLS_AFTER, page::RECEIPT_CUT_PERCENT);
    if !html.contains(&facts.pin.sha) {
        problems.push("the pinned commit is not on the page".to_string());
    }
    if facts.receipt.state == "success" && !html.contains(">PASS<") {
        problems.push("a passing verdict is not shown as PASS".to_string());
    }
    if facts.receipt.state == "failure" && !html.contains(">FAIL<") && !html.contains(">BLOCKED<") {
        problems.push("a failing verdict is not shown as FAIL or BLOCKED".to_string());
    }
    if problems.is_empty() {
        println!("verified {} ({} bytes); receipt figures: {status_md_hint}", dist.join("index.html").display(), html.len());
        Ok(())
    } else {
        Err(problems.join("\n"))
    }
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mode = args.first().map(String::as_str).unwrap_or("build");
    let result = match mode {
        "facts" => {
            let out = PathBuf::from(args.get(1).map(String::as_str).unwrap_or("facts.json"));
            facts::gather().and_then(|f| {
                let r = f.save(&out);
                if r.is_ok() {
                    println!("facts: {}@{} tag {} receipt {} -> {}", f.pin.full_name, &f.pin.sha[..12.min(f.pin.sha.len())], f.pin.latest_tag, f.receipt.state, out.display());
                }
                r
            })
        }
        "build" => build(Path::new(args.get(1).map(String::as_str).unwrap_or("dist")), Path::new(args.get(2).map(String::as_str).unwrap_or("facts.json"))),
        "verify" => verify(Path::new(args.get(1).map(String::as_str).unwrap_or("dist"))),
        other => Err(format!("unknown mode {other}; use facts, build or verify")),
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("site-builder {mode}: {e}");
            ExitCode::FAILURE
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use facts::*;

    fn sample() -> Facts {
        Facts {
            generated_at: "2026-10-16T17:00:00Z".into(),
            pin: Pin {
                full_name: "aien-dev/interplane".into(),
                url: "https://github.com/aien-dev/interplane".into(),
                sha: "83df0f074b2c0000000000000000000000000000".into(),
                committed_at: "2026-10-16T09:30:35Z".into(),
                commit_title: "ci: enroll interplane in the Spark verification pipeline (#109)".into(),
                latest_tag: "v0.3.0".into(),
                license: "Apache-2.0".into(),
                description: "d".into(),
            },
            status: StatusCounts { rows: 40, qualified: 4, conformant: 12, tested: 10, implemented: 2, specified: 1, other: 11 },
            conformance: Some(Conformance { passed: 106, total: 106, note: "106/106".into() }),
            stages: vec![
                Stage { version: "0.1".into(), layer: "Reality".into(), goal: "Prove".into(), state: "reported".into(), verdict: String::new(), closed_on: String::new(), report_url: "https://x/REPORT-0.1.md".into() },
                Stage { version: "0.3".into(), layer: "Trust".into(), goal: "Keep".into(), state: "closed".into(), verdict: "PASS".into(), closed_on: "2026-10-05".into(), report_url: "https://x/REPORT-0.3.md".into() },
                Stage { version: "0.4".into(), layer: "Execution".into(), goal: "Multi".into(), state: "next".into(), verdict: String::new(), closed_on: String::new(), report_url: String::new() },
            ],
            next_stage: "0.4 Execution".into(),
            reports: vec![Report { name: "REPORT-0.3.md".into(), title: "INTERPLANE 0.3 report".into(), headline: "every offline gate PASSED.".into(), url: "https://x/REPORT-0.3.md".into() }],
            open_issues: 2,
            open_prs: 1,
            last_merge: Some(Merge { number: 109, title: "ci: enroll".into(), merged_at: "2026-10-16T09:30:36Z".into(), url: "https://x/pull/109".into() }),
            receipt: Receipt { state: "success".into(), description: "PASS 7 gates".into(), target_url: "https://x/commit/83df".into(), updated_at: "2026-10-16T10:00:00Z".into() },
        }
    }

    #[test]
    fn rendered_page_passes_the_site_rules() {
        let html = page::render(&sample());
        let css = fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../public/style.css")).expect("public/style.css");
        let problems = verify_html(&html, &css);
        assert!(problems.is_empty(), "{problems:?}");
        assert!(html.contains(">PASS<"));
        assert!(html.contains("83df0f074b2c"));
        assert!(html.contains("106 of 106"));
        assert!(html.contains("v0.3.0"));
    }

    #[test]
    fn discoverability_is_inert_pinned_and_html_safe() {
        let mut f = sample();
        f.pin.description = "literal </script><script>alert(1)</script> & text".into();
        let html = page::render(&f);
        let prefix = "<script type=\"application/ld+json\">";
        let (_, rest) = html.split_once(prefix).expect("missing structured data");
        let (raw, _) = rest.split_once("</script>").unwrap();
        let data: serde_json::Value = serde_json::from_str(raw).expect("valid JSON-LD");
        assert_eq!(data["@context"], "https://schema.org");
        let graph = data["@graph"].as_array().unwrap();
        assert_eq!(graph.len(), 2);
        assert_eq!(graph[0]["@type"], "WebSite");
        assert_eq!(graph[1]["@type"], "SoftwareSourceCode");
        assert_eq!(graph[1]["version"], f.pin.sha);
        assert_eq!(graph[1]["codeRepository"], f.pin.url);
        assert_eq!(graph[1]["description"], f.pin.description);
        assert!(!raw.contains('<') && !raw.contains('&'));
        assert!(verify_html(&html, "").is_empty());
        assert!(!verify_html(&(html.clone() + "<SCRIPT>alert(1)</SCRIPT>"), "").is_empty());
        assert!(!verify_html(&html.replace(raw, "not json"), "").is_empty());
        assert!(html.contains("class=\"language-sh\""));
        assert!(html.contains("main id=\"main\" tabindex=\"-1\""), "skip target must be focusable");
    }

    #[test]
    fn built_discovery_files_and_corruption_are_verified() {
        let dir = std::env::temp_dir().join(format!("interplane-discovery-{}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        let facts_path = dir.join("input.json");
        let f = sample();
        f.save(&facts_path).unwrap();
        let out = dir.join("dist");
        // CLI build is run at repository root; Cargo runs unit tests in the crate directory.
        std::env::set_current_dir(concat!(env!("CARGO_MANIFEST_DIR"), "/../..")).unwrap();
        build(&out, &facts_path).unwrap();
        let llms = fs::read_to_string(out.join("llms.txt")).expect("missing llms.txt");
        assert!(llms.contains(page::SITE_URL) && llms.contains(&f.blob("STATUS.md")));
        assert!(!fs::read_to_string(out.join("sitemap.xml")).unwrap().contains("<lastmod>"));
        assert!(verify(&out).is_ok());
        for file in ["llms.txt", "robots.txt", "sitemap.xml", "public/hero.webp"] {
            let path = out.join(file);
            let original = fs::read(&path).unwrap();
            fs::remove_file(&path).unwrap();
            assert!(verify(&out).is_err(), "missing {file} must be refused");
            fs::write(path, original).unwrap();
        }
        let index = out.join("index.html");
        let original = fs::read_to_string(&index).unwrap();
        for (from, to) in [
            ("rel=\"canonical\" href=\"https://interplane.io/\"", "rel=\"canonical\" href=\"https://wrong.example/\""),
            ("https://interplane.io/public/hero.webp", "https://wrong.example/missing.webp"),
            ("href=\"facts.json\"", "href=\"missing.json\""),
            ("SoftwareSourceCode", "FakeSoftware"),
        ] {
            assert!(original.contains(from));
            fs::write(&index, original.replace(from, to)).unwrap();
            assert!(verify(&out).is_err(), "corruption {from} must be refused");
        }
        fs::write(&index, original.clone()).unwrap();
        for duplicate in [
            "<link rel=\"canonical\" href=\"https://attacker.example/\">",
            "<meta property=\"og:image\" content=\"https://attacker.example/image.webp\">",
            "<LINK REL='canonical' HREF='https://attacker.example/'>",
            "<META PROPERTY='og:image' CONTENT='https://attacker.example/image.webp'>",
        ] {
            for mutated in [format!("{original}{duplicate}"), original.replace("</head>", &format!("{duplicate}</head>"))] {
                fs::write(&index, mutated).unwrap();
                assert!(verify(&out).is_err(), "conflicting metadata must be refused: {duplicate}");
            }
        }
        fs::write(&index, original).unwrap();
        assert!(verify(&out).is_ok());
        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn failing_verdict_is_red_and_honest() {
        let mut f = sample();
        f.receipt.state = "failure".into();
        f.receipt.description = "BLOCKED NOT_RUN PIPELINE_CONFIG_CHANGED".into();
        let html = page::render(&f);
        assert!(html.contains("pill bad"));
        assert!(html.contains(">BLOCKED<"), "a run the judge did not run says BLOCKED, not FAIL");
        assert!(html.contains("BLOCKED NOT_RUN PIPELINE_CONFIG_CHANGED"));
        f.receipt.description = "BLOCKED FAIL PREFILL-E2E-2:rc_nonzero".into();
        let html = page::render(&f);
        assert!(html.contains(">FAIL<"), "a failed gate says FAIL even when the bundle is also blocked");
    }

    #[test]
    fn no_verdict_is_said_plainly() {
        let mut f = sample();
        f.receipt = Receipt { state: "none".into(), description: "The Spark has not published a verdict for this commit yet.".into(), target_url: String::new(), updated_at: String::new() };
        let html = page::render(&f);
        assert!(html.contains(">NO VERDICT<"));
        assert!(html.contains("not published a verdict"));
    }
}
