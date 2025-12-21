
// ctas7-phd-analyzer/bin/post_stats.rs  (<=200 loc)
use serde::Deserialize;
use std::fs;
use std::io::{self, Read};

#[derive(Deserialize)]
struct Halstead { volume: f64, difficulty: f64, effort: f64 }
#[derive(Deserialize)]
struct FileMetrics {
    path: String, loc: usize, lloc: usize, comments: usize, cyclo: usize,
    halstead: Halstead, mi: f64, warnings: Vec<String>,
}
#[derive(Deserialize)]
struct Totals { files: usize, loc: usize, lloc: usize, cyclo: usize }
#[derive(Deserialize)]
struct Report { ts: String, root: String, totals: Totals, files: Vec<FileMetrics> }

fn main() -> anyhow::Result<()> {
    let mut args = std::env::args().skip(1);
    let mut url = String::from("http://localhost:18109/ingest");
    let mut token: Option<String> = None;
    let mut source = String::from("ctas7-phd-analyzer");
    let mut tag = String::from("run");
    let mut input_path: Option<String> = None;
    while let Some(a) = args.next() {
        match a.as_str() {
            "--url" => url = args.next().unwrap_or(url),
            "--token" => token = Some(args.next().unwrap_or_default()),
            "--source" => source = args.next().unwrap_or(source),
            "--tag" => tag = args.next().unwrap_or(tag),
            p if !p.starts_with('-') => input_path = Some(p.to_string()),
            _ => {}
        }
    }
    let json = match input_path {
        Some(p) => fs::read_to_string(p)?,
        None => { let mut s=String::new(); io::stdin().read_to_string(&mut s)?; s }
    };
    let rep: Report = serde_json::from_str(&json)?;

    let mut feats: Vec<serde_json::Value> = Vec::new();
    for f in &rep.files {
        let comment_ratio = if f.lloc>0 { f.comments as f64 / f.lloc as f64 } else {0.0};
        let cyclo_per_100 = if f.loc>0 { (f.cyclo as f64) * 100.0 / f.loc as f64 } else {0.0};
        feats.push(serde_json::json!({
            "source": &source, "tag": &tag, "ts": &rep.ts, "path": f.path,
            "vector": { "loc": f.loc, "lloc": f.lloc, "comments": f.comments,
              "comment_ratio": comment_ratio, "cyclo": f.cyclo,
              "cyclo_per_100loc": cyclo_per_100, "mi": f.mi,
              "h_volume": f.halstead.volume, "h_difficulty": f.halstead.difficulty,
              "h_effort": f.halstead.effort, "warns": f.warnings.len() }
        }));
    }
    let avg_cyclo = if rep.totals.files>0 {
        rep.totals.cyclo as f64 / rep.totals.files as f64 } else { 0.0 };
    let summary = serde_json::json!({
        "source": &source, "tag": &tag, "ts": &rep.ts, "root": &rep.root,
        "summary": { "files": rep.totals.files, "loc": rep.totals.loc,
                     "lloc": rep.totals.lloc, "avg_cyclo": avg_cyclo }
    });

    let body = serde_json::json!({
        "kind": "ctas.analysis.features.v1",
        "summary": summary,
        "features": feats
    });
    let client = reqwest::blocking::Client::new();
    let mut req = client.post(&url).json(&body);
    if let Some(t) = token { req = req.bearer_auth(t); }
    match req.send() {
        Ok(resp) if resp.status().is_success() => {
            println!("posted {} features to {}", 
                body["features"].as_array().map(|a|a.len()).unwrap_or(0), url);
        }
        Ok(resp) => eprintln!("CDN replied {}", resp.status()),
        Err(e) => eprintln!("post error: {}", e),
    }
    Ok(())
}
