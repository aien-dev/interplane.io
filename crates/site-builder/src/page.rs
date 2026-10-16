//! The one page of interplane.io. Editorial copy lives here in Rust; every figure comes from
//! `Facts` (read from the protocol repository at a pinned commit) or from a constant below that
//! names the report it was read from.

use maud::{html, Markup, DOCTYPE};

use crate::facts::{short_date, Facts};

pub const TITLE: &str = "INTERPLANE";
pub const TAGLINE: &str = "The open boundary between AI models and the programs that act for them.";
pub const DESCRIPTION: &str = "INTERPLANE is an open interoperability protocol between AI models and agent runtimes. Models express intent. Runtimes retain authority.";
pub const SITE_URL: &str = "https://interplane.io/";
pub const SITE_REPO: &str = "https://github.com/aien-dev/interplane.io";

/// The 0.1 receipt, read from docs/REPORT-0.1.md ("Selection" section) at the pinned commit.
/// Only the report path is cited on the page; the numbers below are copied from it by hand and
/// checked by `verify` against the facts file's STATUS row for CrossAxis Select.
pub const RECEIPT_TOOLS_BEFORE: u32 = 71;
pub const RECEIPT_TOOLS_AFTER: u32 = 9;
pub const RECEIPT_TOKENS_BEFORE: u32 = 15_846;
pub const RECEIPT_TOKENS_AFTER: u32 = 1_674;
pub const RECEIPT_CUT_PERCENT: &str = "89";
pub const RECEIPT_REPORT_PATH: &str = "docs/REPORT-0.1.md";

pub struct Module {
    pub name: &'static str,
    pub question: &'static str,
    pub answer: &'static str,
    pub spec: &'static str,
}

/// The modules, in the order a call crosses the boundary.
pub const MODULES: [Module; 7] = [
    Module { name: "Core", question: "What crosses the line?", answer: "The shared vocabulary: capabilities, requests, results, traces and the canonical byte form every digest is computed over.", spec: "spec/CORE.md" },
    Module { name: "Lenshift", question: "What did the model actually say?", answer: "Reads each model's native tool-call dialect into one form. An incomplete or ambiguous call is rejected, never guessed.", spec: "spec/LENSHIFT.md" },
    Module { name: "CrossAxis", question: "Which tools does this task need?", answer: "Selects the few tools a task needs from a runtime's catalog, so a small local model is not buried under schemas it will never use.", spec: "spec/CROSSAXIS.md" },
    Module { name: "Crossveil", question: "Who is allowed to act?", answer: "Tracks where every piece of text came from. Content a model read on the web cannot grant authority, and an effect it asks for while untrusted text is in view is held for a person.", spec: "spec/CROSSVEIL.md" },
    Module { name: "Probe", question: "What does this model and backend really support?", answer: "Live probes that measure a model, backend and runtime combination instead of trusting its documentation.", spec: "spec/PROBE.md" },
    Module { name: "Vectorveil", question: "How do bytes travel?", answer: "The transport envelope. Specified; the implementation belongs to the 0.4 Execution stage.", spec: "spec/VECTORVEIL.md" },
    Module { name: "RelayLine", question: "How does a long call stream?", answer: "Streaming, cancellation and continuation. The contract is written; the implementation belongs to 0.4.", spec: "spec/RELAYLINE.md" },
];

fn number(n: u32) -> String {
    let s = n.to_string();
    let mut out = String::new();
    for (i, ch) in s.chars().enumerate() {
        if i > 0 && (s.len() - i) % 3 == 0 {
            out.push('\u{202f}');
        }
        out.push(ch);
    }
    out
}

fn head(facts: &Facts) -> Markup {
    html! {
        head {
            meta charset="utf-8";
            meta name="viewport" content="width=device-width, initial-scale=1";
            title { (TITLE) ": " (TAGLINE) }
            meta name="description" content=(DESCRIPTION);
            meta property="og:title" content=(TITLE);
            meta property="og:description" content=(DESCRIPTION);
            meta property="og:type" content="website";
            meta property="og:url" content=(SITE_URL);
            meta property="og:image" content={ (SITE_URL) "public/hero.webp" };
            meta name="color-scheme" content="light dark";
            meta name="generator" content={ "site-builder at " (facts.pin.full_name) "@" (&facts.pin.sha[..12.min(facts.pin.sha.len())]) };
            link rel="canonical" href=(SITE_URL);
            link rel="icon" href="data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 32 32'%3E%3Crect width='32' height='32' rx='6' fill='%23101317'/%3E%3Crect x='6' y='15' width='20' height='2' fill='%23f2a93b'/%3E%3C/svg%3E";
            link rel="stylesheet" href="public/style.css";
            link rel="alternate" type="application/json" href="facts.json" title="Facts behind this page";
        }
    }
}

fn verdict_class(state: &str) -> &'static str {
    match state {
        "success" => "ok",
        "failure" => "bad",
        "pending" => "wait",
        _ => "none",
    }
}

/// The word in the verdict pill. The Spark reports a blocked run (for example a pipeline
/// configuration change that needs an operator run) as a failing status; the page says BLOCKED
/// for those so a reader does not take it for a failed test.
pub fn verdict_word(receipt: &crate::facts::Receipt) -> &'static str {
    match receipt.state.as_str() {
        "success" => "PASS",
        "failure" if receipt.description.starts_with("BLOCKED") && receipt.description.contains("NOT_RUN") => "BLOCKED",
        "failure" => "FAIL",
        "pending" => "RUNNING",
        _ => "NO VERDICT",
    }
}

fn hero(facts: &Facts) -> Markup {
    let release = if facts.pin.latest_tag.is_empty() { "unreleased".to_string() } else { facts.pin.latest_tag.clone() };
    html! {
        header class="hero" {
            div class="hero-text" {
                p class="kicker" { "Open protocol" " " span class="dot" {} " " (release) " " span class="dot" {} " " (facts.pin.license) }
                h1 { (TITLE) }
                p class="tagline" { (TAGLINE) }
                p class="lede" {
                    "A model can say what it wants done. Only the program running it decides what happens. INTERPLANE is the written agreement between the two sides, with a test suite that checks every implementation the same way."
                }
                p class="actions" {
                    a class="button primary" href=(facts.pin.url) { "Read the source" }
                    a class="button" href=(facts.blob("spec/CORE.md")) { "Read the specification" }
                    a class="button" href="#status" { "See the current state" }
                }
            }
            figure class="hero-art" {
                img src="public/hero.webp" width="1536" height="1024" alt="Two machined metal plates held apart by struts, with a single luminous amber plane stretched between them: the INTERPLANE layer between a model and a runtime." loading="eager";
            }
        }
    }
}

fn problem() -> Markup {
    html! {
        section class="prose" id="why" {
            h2 { "The problem it fixes" }
            p { "Every local model speaks its own tool-calling dialect. Every agent program has its own tools, approvals and memory. Today each pair is wired by hand, and the wiring is where the authority leaks: a model reads a web page that contains an instruction, repeats it as a tool call, and the program obeys because nothing in between remembered where the text came from." }
            p { "INTERPLANE puts a named, tested layer there. It belongs to neither side. A runtime keeps its own policy and execution; a model keeps its own dialect. The protocol only standardises what crosses between them, and makes that crossing measurable." }
            blockquote { "Models express intent. Runtimes retain authority. INTERPLANE defines the boundary between them." }
        }
    }
}

fn modules(facts: &Facts) -> Markup {
    html! {
        section id="modules" {
            h2 { "Seven modules, one line" }
            p class="section-lede" { "Each module answers one question a call has to pass on its way from a model to an effect in the world." }
            ol class="modules" {
                @for m in MODULES.iter() {
                    li {
                        h3 { (m.name) }
                        p class="question" { (m.question) }
                        p { (m.answer) }
                        a class="spec" href=(facts.blob(m.spec)) { "Specification" }
                    }
                }
            }
        }
    }
}

fn receipt(facts: &Facts) -> Markup {
    html! {
        section id="receipt" class="receipt" {
            h2 { "One measured result" }
            p class="section-lede" { "From the 0.1 readiness report. A real Qwen3.5-9B model on Ollama, one task, the runtime's full tool catalog against the tools CrossAxis selected." }
            div class="receipt-grid" {
                div class="cell" { span class="big" { (RECEIPT_TOOLS_BEFORE) " to " (RECEIPT_TOOLS_AFTER) } span class="label" { "tools offered for the task" } }
                div class="cell" { span class="big" { (number(RECEIPT_TOKENS_BEFORE)) " to " (number(RECEIPT_TOKENS_AFTER)) } span class="label" { "first-turn prompt tokens, model reported" } }
                div class="cell" { span class="big" { (RECEIPT_CUT_PERCENT) "% fewer" } span class="label" { "prompt tokens before the model says a word" } }
            }
            p class="fine" {
                "Fewer tokens is the cost side only. The same stage later asked whether a small model keeps its capability with fewer tools, pre-registered the thresholds, and recorded the answer: the 0.2 gate "
                strong { "failed" }
                ". Both results are in the repository, with the raw runs. "
                a href=(facts.blob(RECEIPT_REPORT_PATH)) { "Read the 0.1 report" }
                "."
            }
        }
    }
}

fn status(facts: &Facts) -> Markup {
    let c = &facts.status;
    html! {
        section id="status" class="status" {
            h2 { "Current state" }
            p class="section-lede" {
                "Read from the protocol repository at commit "
                a href={ (facts.pin.url) "/commit/" (facts.pin.sha) } { code { (&facts.pin.sha[..12.min(facts.pin.sha.len())]) } }
                " (" (short_date(&facts.pin.committed_at)) "). This page is rebuilt every hour from that record and nothing here is typed by hand."
            }
            div class="verdict" {
                span class={ "pill " (verdict_class(&facts.receipt.state)) } { (verdict_word(&facts.receipt)) }
                span class="verdict-text" {
                    "Spark verification of this commit: " (facts.receipt.description)
                    @if !facts.receipt.target_url.is_empty() { " " a href=(facts.receipt.target_url) { "Receipt" } }
                }
            }
            div class="stages" {
                h3 { "Roadmap" }
                ol class="stage-list" {
                    @for st in &facts.stages {
                        li class={ "stage " (st.state) } {
                            span class="version" { (st.version) }
                            span class="layer" { (st.layer) }
                            span class="goal" { (st.goal) }
                            span class="state" {
                                @match st.state.as_str() {
                                    "closed" => {
                                        "closed " (short_date(&st.closed_on))
                                        @if !st.verdict.is_empty() { ", gate " strong class=(if st.verdict == "PASS" { "ok" } else { "bad" }) { (st.verdict) } }
                                    }
                                    "next" => "next",
                                    "reported" => "closed, report below",
                                    _ => "planned",
                                }
                                @if !st.report_url.is_empty() { " " a href=(st.report_url) { "Report" } }
                            }
                        }
                    }
                }
                @if !facts.next_stage.is_empty() { p class="fine" { "Next stage: " strong { (facts.next_stage) } "." } }
            }
            div class="status-grid" {
                div class="cell" {
                    h3 { "Components" }
                    p { "STATUS.md tracks " strong { (c.rows) } " rows for the Rust implementation." }
                    ul class="counts" {
                        li { strong { (c.qualified) } " qualified against a real model or backend" }
                        li { strong { (c.conformant) } " conformant to the shared fixtures" }
                        li { strong { (c.tested) } " tested" }
                        li { strong { (c.implemented) } " implemented or running" }
                        li { strong { (c.specified) } " specified only" }
                    }
                    @if let Some(conf) = &facts.conformance {
                        p { "Conformance runner: " strong { (conf.passed) " of " (conf.total) } " cases, verdicts byte-identical to the recorded reference." }
                    }
                    a href=(facts.blob("STATUS.md")) { "Full status table" }
                }
                div class="cell" {
                    h3 { "Activity" }
                    ul class="counts" {
                        li { "Latest change: " a href={ (facts.pin.url) "/commit/" (facts.pin.sha) } { (facts.pin.commit_title) } }
                        @if let Some(m) = &facts.last_merge {
                            li { "Last merged proposal: " a href=(m.url) { "#" (m.number) " " (m.title) } " (" (short_date(&m.merged_at)) ")" }
                        }
                        li { strong { (facts.open_prs) } " open proposals, " strong { (facts.open_issues) } " open issues" }
                        li { "Latest release: " strong { (if facts.pin.latest_tag.is_empty() { "none".to_string() } else { facts.pin.latest_tag.clone() }) } }
                    }
                    a href={ (facts.pin.url) "/pulls" } { "Proposals" } " " span class="dot" {} " " a href={ (facts.pin.url) "/issues" } { "Issues" }
                }
            }
            div class="reports" {
                h3 { "Reports, in their own words" }
                p class="section-lede" { "Every stage closes with a pre-registered gate and a report that states the result whether it passed or failed." }
                ul class="report-list" {
                    @for r in &facts.reports {
                        li {
                            a href=(r.url) { (r.title) }
                            @if !r.headline.is_empty() { span class="headline" { " " (r.headline) } }
                        }
                    }
                }
            }
            p class="fine" {
                "Generated " (facts.generated_at) ". The data behind this section is published as "
                a href="facts.json" { "facts.json" } "."
            }
        }
    }
}

fn get_started(facts: &Facts) -> Markup {
    html! {
        section id="start" class="prose" {
            h2 { "Try it in a minute, offline" }
            p { "No model, no GPU, no network. A scripted model fetches a page that carries an injected instruction, then asks to send an email, write a file and append a note. The reference pipeline denies the email and the write, holds the note for a person, and executes none of them." }
            pre { code { "git clone " (facts.pin.url) ".git\ncd interplane/rust\ncargo run -q -p interplane-crossveil --example offline" } }
            p { "To connect your own program, read " a href=(facts.blob("docs/WRITING-AN-ADAPTER.md")) { "Writing an adapter" } " and run its to-do list example. The conformance suite then checks your adapter the same way it checks the reference." }
        }
    }
}

fn footer(facts: &Facts) -> Markup {
    html! {
        footer {
            p { "INTERPLANE is developed in the open by " a href="https://github.com/aien-dev" { "aien-dev" } " as part of " a href="https://www.aienos.com/" { "AIEN" } ". Protocol and reference implementation: " (facts.pin.license) ". " a href=(SITE_REPO) { "This site" } " is built by a small Rust program from the repository's public record; no scripts, no outside fonts, no trackers." }
        }
    }
}

pub fn render(facts: &Facts) -> String {
    let page = html! {
        (DOCTYPE)
        html lang="en" {
            (head(facts))
            body {
                a class="skip" href="#main" { "Skip to content" }
                (hero(facts))
                main id="main" {
                    (problem())
                    (modules(facts))
                    (receipt(facts))
                    (status(facts))
                    (get_started(facts))
                }
                (footer(facts))
            }
        }
    };
    page.into_string()
}
