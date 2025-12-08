// ctas7-quality-kit/src/loc_board.rs
use chrono::Utc;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, fs, path::Path};

#[derive(Serialize, Deserialize)]
pub struct Snapshot {
    pub ts: String,
    pub crates: BTreeMap<String, usize>,
}

fn is_rs(path: &std::path::Path) -> bool {
    path.extension().map(|e| e == "rs").unwrap_or(false)
}

fn count_file(p: &std::path::Path) -> usize {
    if let Ok(s) = fs::read_to_string(p) {
        s.lines()
         .filter(|l| !l.trim().is_empty() && !l.trim_start().starts_with("//"))
         .count()
    } else { 0 }
}

fn crate_loc(root: &str) -> BTreeMap<String, usize> {
    let mut out = BTreeMap::new();
    for e in walkdir::WalkDir::new(root).into_iter().filter_map(Result::ok) {
        let p = e.path();
        if !e.file_type().is_file() || !is_rs(p) { continue; }
        let crate_name = p.ancestors()
            .find(|a| a.join("Cargo.toml").exists())
            .and_then(|a| a.file_name()).and_then(|s| s.to_str())
            .unwrap_or("unknown").to_string();
        *out.entry(crate_name).or_insert(0) += count_file(p);
    }
    out
}

pub fn snapshot(root: &str, out_path: &str) -> anyhow::Result<()> {
    let map = crate_loc(root);
    let snap = Snapshot { ts: Utc::now().to_rfc3339(), crates: map };
    fs::write(out_path, serde_json::to_vec_pretty(&snap)?)?;
    Ok(())
}

pub fn report(today: &Snapshot, yesterday: Option<&Snapshot>) {
    let total: usize = today.crates.values().sum();
    let crates = today.crates.len();
    println!("LOC TODAY: {} in {} crates", total, crates);
    if let Some(y) = yesterday {
        let ytot: usize = y.crates.values().sum();
        let delta = total as isize - ytot as isize;
        println!("LOC YESTERDAY: {}  Δ{}", ytot, delta);
        let files_guess = (delta.abs() / 200.max(1) as isize).max(1) as usize;
        if (delta.abs() as usize) > files_guess * 500 {
            println!("⚠️  anomaly: delta too large for expected file count");
        }
    }
}

pub fn load(path: &str) -> Option<Snapshot> {
    Path::new(path).exists()
        .then(|| fs::read(path).ok()).flatten()
        .and_then(|b| serde_json::from_slice(&b).ok())
}
