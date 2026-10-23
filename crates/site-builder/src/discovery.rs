//! Discovery metadata, generated from the same facts as the visible page.
use crate::{facts::Facts, page};
use serde_json::{json, Value};
use std::{fs, path::Path};

pub const JSON_OPEN: &str = "<script type=\"application/ld+json\">";

pub fn structured_data(f: &Facts) -> Value {
    json!({"@context":"https://schema.org", "@graph":[
        {"@type":"WebSite", "@id":page::SITE_URL, "url":page::SITE_URL,
         "name":page::TITLE, "description":page::DESCRIPTION},
        {"@type":"SoftwareSourceCode", "name":"INTERPLANE protocol",
         "codeRepository":f.pin.url, "version":f.pin.sha,
         "description":f.pin.description}
    ]})
}

pub fn script(f: &Facts) -> String {
    // HTML raw-text parsing ignores JSON quoting: never allow a literal '<' to close the tag.
    let raw = structured_data(f)
        .to_string()
        .replace('&', "\\u0026")
        .replace('<', "\\u003c")
        .replace('>', "\\u003e")
        .replace('\u{2028}', "\\u2028")
        .replace('\u{2029}', "\\u2029");
    format!("{JSON_OPEN}{raw}</script>")
}

pub fn json_block(html: &str) -> Result<Value, String> {
    let lower = html.to_ascii_lowercase();
    if lower.matches("<script").count() != 1 || lower.matches("</script").count() != 1 {
        return Err("expected exactly one inert JSON-LD block and no executable scripts".into());
    }
    let (_, rest) = html
        .split_once(JSON_OPEN)
        .ok_or("only exact inert application/ld+json scripts are allowed")?;
    let (raw, _) = rest
        .split_once("</script>")
        .ok_or("unclosed JSON-LD block")?;
    if raw.contains('<') || raw.contains('&') {
        return Err("unsafe HTML delimiter in JSON-LD".into());
    }
    let value: Value = serde_json::from_str(raw).map_err(|e| format!("invalid JSON-LD: {e}"))?;
    if value["@context"] != "https://schema.org" || !value["@graph"].is_array() {
        return Err("invalid JSON-LD graph".into());
    }
    Ok(value)
}

pub fn robots() -> String {
    format!(
        "User-agent: *\nAllow: /\nSitemap: {}sitemap.xml\n",
        page::SITE_URL
    )
}

pub fn sitemap() -> String {
    // The fetch timestamp is not a content modification date. Do not invent lastmod.
    format!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\n  <url><loc>{}</loc></url>\n</urlset>\n", page::SITE_URL)
}

pub fn llms(f: &Facts) -> String {
    format!("# INTERPLANE\n\n> {}\n\nExperimental protocol. Models express intent; runtimes retain authority. Implementation, planned work and qualification are distinguished in the pinned public record.\n\n## Current website\n- [Overview]({}): the current single-page site, including offline example and status.\n- [Generated facts]({}facts.json): the source pin and unmodified verification state behind the page.\n\n## Pinned protocol documentation\n- [Status]({}): supported and qualified components.\n- [Writing an adapter]({}): canonical adapter instructions.\n- [Roadmap]({}): implemented versus planned stages.\n\nProtocol source revision: {}. This index is discovery metadata, not a new support or security guarantee.\n", page::DESCRIPTION, page::SITE_URL, page::SITE_URL, f.blob("STATUS.md"), f.blob("docs/WRITING-AN-ADAPTER.md"), f.blob("ROADMAP.md"), f.pin.sha)
}

pub fn write(dist: &Path, f: &Facts) -> Result<(), String> {
    for (name, text) in [
        ("robots.txt", robots()),
        ("sitemap.xml", sitemap()),
        ("llms.txt", llms(f)),
    ] {
        fs::write(dist.join(name), text).map_err(|e| format!("{name}: {e}"))?;
    }
    Ok(())
}

pub fn verify(dist: &Path, html: &str, f: &Facts) -> Vec<String> {
    let mut problems = Vec::new();
    match json_block(html) {
        Ok(value) if value == structured_data(f) => (),
        _ => problems.push("JSON-LD does not match the pinned visible facts".into()),
    }
    for (name, expected) in [
        ("robots.txt", robots()),
        ("sitemap.xml", sitemap()),
        ("llms.txt", llms(f)),
    ] {
        if fs::read_to_string(dist.join(name)).ok().as_deref() != Some(&expected) {
            problems.push(format!("{name} is missing or inconsistent"));
        }
    }
    for marker in [
        format!("rel=\"canonical\" href=\"{}\"", page::SITE_URL),
        format!(
            "property=\"og:image\" content=\"{}public/hero.webp\"",
            page::SITE_URL
        ),
    ] {
        if !html.contains(&marker) {
            problems.push(format!("missing or wrong metadata: {marker}"));
        }
    }
    // Maud writes quoted attributes. Check every generated local href/src, including fragments.
    for attr in ["href=\"", "src=\""] {
        for rest in html.split(attr).skip(1) {
            let Some((url, _)) = rest.split_once('"') else {
                problems.push("unclosed link".into());
                continue;
            };
            if url.starts_with("https://") || url.starts_with("data:") {
                continue;
            }
            if let Some(id) = url.strip_prefix('#') {
                if !html.contains(&format!("id=\"{id}\"")) {
                    problems.push(format!("missing fragment {url}"));
                }
                continue;
            }
            if url.is_empty()
                || url.contains(':')
                || url.starts_with('/')
                || url.split('/').any(|p| p == "..")
            {
                problems.push(format!("unsafe local target {url}"));
            } else if !dist.join(url).is_file() {
                problems.push(format!("missing local target {url}"));
            }
        }
    }
    problems
}
