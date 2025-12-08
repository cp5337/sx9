// ctas7-quality-kit/src/repo_prompt.rs
use serde::Serialize;

#[derive(Serialize)]
pub struct RepoPrompt {
    pub system: String,
    pub constraints: Vec<String>,
    pub context: String,
    pub ask: String,
}

pub fn make(
    eei_open: usize,
    crates_brief: &str,
    risks: &[&str],
    request: &str,
) -> RepoPrompt {
    let sys = "You are the CTAS lead engineer. Be precise. Prefer modules (≤200 LOC, 90 cols). No made‑up code.";
    let mut cons = vec![
        "Only hashes traverse; no payloads.".into(),
        "Unsafe Rust = 0 unless justified.".into(),
        "Features behind flags (ooda,xsd,playbookN).".into(),
        "USIM header; n‑v‑n‑n comments.".into(),
    ];
    cons.extend(risks.iter().map(|r| format!("Mitigate: {}", r)));
    let ctx = format!("Open EEIs: {} | Crates: {}", eei_open, crates_brief);
    RepoPrompt { system: sys.into(), constraints: cons, context: ctx,
                 ask: request.into() }
}
