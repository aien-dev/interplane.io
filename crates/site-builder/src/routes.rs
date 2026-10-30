//! The complete generated route inventory. No remote route discovery.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Route {
    Overview,
    QuickStart,
    Concepts,
    Adapter,
    Conformance,
    Status,
}
pub const ALL: [Route; 6] = [
    Route::Overview,
    Route::QuickStart,
    Route::Concepts,
    Route::Adapter,
    Route::Conformance,
    Route::Status,
];
impl Route {
    pub fn path(self) -> &'static str {
        match self {
            Self::Overview => "/",
            Self::QuickStart => "/quick-start/",
            Self::Concepts => "/concepts/",
            Self::Adapter => "/adapter/",
            Self::Conformance => "/conformance/",
            Self::Status => "/status/",
        }
    }
    pub fn file(self) -> String {
        format!("{}index.html", self.path().trim_start_matches('/'))
    }
    pub fn label(self) -> &'static str {
        match self {
            Self::Overview => "Overview",
            Self::QuickStart => "Quick Start",
            Self::Concepts => "Concepts",
            Self::Adapter => "Writing an Adapter",
            Self::Conformance => "Conformance",
            Self::Status => "Versioned Status",
        }
    }
    pub fn description(self) -> &'static str {
        match self {
        Self::Overview => crate::page::DESCRIPTION,
        Self::QuickStart => "Run INTERPLANE's pinned scripted offline example and inspect denied actions and held approvals without a model or GPU.",
        Self::Concepts => "Understand the model, protocol and runtime boundary: proposed intent is not authority, and the runtime retains execution and approval control.",
        Self::Adapter => "Follow INTERPLANE's canonical pinned Rust adapter guide, host-owned decisions and conformance requirements without assuming universal support.",
        Self::Conformance => "Trace INTERPLANE conformance, refusals and reported results to source-pinned fixtures and evidence, including failed and planned stages.",
        Self::Status => "Read INTERPLANE's versioned source status and exact Spark verification verdict from the canonical generated facts.",
    }
    }
    pub fn url(self) -> String {
        format!(
            "{}{}",
            crate::page::SITE_URL,
            self.path().trim_start_matches('/')
        )
    }
    pub fn title(self) -> String {
        if self == Self::Overview {
            format!("{}: {}", crate::page::TITLE, crate::page::TAGLINE)
        } else {
            format!("{}: {}", crate::page::TITLE, self.label())
        }
    }
}
