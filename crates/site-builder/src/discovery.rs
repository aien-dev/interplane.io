//! Discovery metadata, generated from the same facts and route inventory as visible HTML.
use crate::{
    facts::Facts,
    page,
    routes::{Route, ALL},
};
use serde_json::{json, Value};
use std::{fs, path::Path};
pub const JSON_OPEN: &str = "<script type=\"application/ld+json\">";

pub fn structured_data(f: &Facts, route: Route) -> Value {
    let mut graph = vec![
        json!({"@type":"WebSite", "@id":page::SITE_URL, "url":page::SITE_URL,"name":page::TITLE,"description":page::DESCRIPTION}),
        json!({"@type":"SoftwareSourceCode","name":"INTERPLANE protocol","codeRepository":f.pin.url,"version":f.pin.sha,"description":f.pin.description}),
    ];
    if route != Route::Overview {
        graph.push(json!({"@type":"TechArticle","url":route.url(),"headline":route.title(),"description":route.description(),"isPartOf":{"@id":page::SITE_URL},"citation":f.blob("STATUS.md")}));
    }
    json!({"@context":"https://schema.org","@graph":graph})
}
pub fn script(f: &Facts, route: Route) -> String {
    let raw = structured_data(f, route)
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
    let rows = ALL
        .iter()
        .map(|r| format!("  <url><loc>{}</loc></url>\n", r.url()))
        .collect::<String>();
    // An hourly fetch is not a content-modification date. Do not invent lastmod.
    format!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\n{rows}</urlset>\n")
}
pub fn llms(f: &Facts) -> String {
    let pages = ALL
        .iter()
        .map(|r| format!("- [{}]({}): {}\n", r.label(), r.url(), r.description()))
        .collect::<String>();
    format!("# INTERPLANE\n\n> {}\n\nExperimental protocol. Models express intent; runtimes retain authority. Implemented and qualified behavior is distinct from planned work.\n\n## Current website\n{}- [Generated facts]({}facts.json): the source pin and exact verification state.\n\n## Pinned protocol documentation\n- [Status]({})\n- [Writing an adapter]({})\n- [Roadmap]({})\n\nProtocol source revision: {}. Discovery metadata is not a support or security guarantee.\n",page::DESCRIPTION,pages,page::SITE_URL,f.blob("STATUS.md"),f.blob("docs/WRITING-AN-ADAPTER.md"),f.blob("ROADMAP.md"),f.pin.sha)
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
pub fn verify(dist: &Path, html: &str, f: &Facts, route: Route) -> Vec<String> {
    let mut problems = Vec::new();
    match json_block(html) {
        Ok(value) if value == structured_data(f, route) => (),
        _ => problems.push("JSON-LD does not match the pinned visible facts and route".into()),
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
    let expected = page::render_route(f, route);
    let head = |s: &str| {
        s.split_once("<head>")
            .and_then(|(_, rest)| rest.split_once("</head>").map(|(head, _)| head.to_owned()))
    };
    if head(html).is_none() || head(html) != head(&expected) {
        problems.push("head metadata differs from the generated facts-bound route head".into());
    }
    let lower = html.to_ascii_lowercase();
    let expected_lower = expected.to_ascii_lowercase();
    for tag in ["<head", "</head", "<link", "<meta"] {
        if lower.matches(tag).count() != expected_lower.matches(tag).count() {
            problems.push(format!("extra or missing metadata tag {tag}"));
        }
    }
    let page_dir = dist.join(route.file()).parent().unwrap().to_path_buf();
    for attr in ["href=\"", "src=\""] {
        for rest in html.split(attr).skip(1) {
            let Some((url, _)) = rest.split_once('"') else {
                problems.push("unclosed link".into());
                continue;
            };
            if url.starts_with("https://") || url.starts_with("data:") {
                continue;
            }
            let (path, fragment) = url
                .split_once('#')
                .map_or((url, None), |(p, f)| (p, Some(f)));
            if url.is_empty()
                || path.contains(':')
                || path.starts_with("//")
                || path.split('/').any(|p| p == "..")
            {
                problems.push(format!("unsafe local target {url}"));
                continue;
            }
            let mut target = if path.is_empty() {
                dist.join(route.file())
            } else if path.starts_with('/') {
                dist.join(path.trim_start_matches('/'))
            } else {
                page_dir.join(path)
            };
            if target.is_dir() {
                target = target.join("index.html");
            }
            if !target.is_file() {
                problems.push(format!("missing local target {url}"));
                continue;
            }
            if let Some(id) = fragment {
                let contents = if path.is_empty() {
                    Some(html.to_owned())
                } else {
                    fs::read_to_string(target).ok()
                };
                if !contents.is_some_and(|s| s.contains(&format!("id=\"{id}\""))) {
                    problems.push(format!("missing fragment {url}"));
                }
            }
        }
    }
    problems
}
