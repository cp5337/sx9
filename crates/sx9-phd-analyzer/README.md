# CTAS7 PhD Analyzer (with Statistical CDN)

This crate provides:
- `ctas7-phd-analyzer` (binary): computes cyclomatic, Halstead, MI, doc density.
- `post_stats` (binary): posts feature vectors to the Statistical Analysis CDN.
- `scripts/phd_suite.sh`: runs clippy/geiger/audit/coverage/tokei.
- `scripts/send_to_stats.sh`: analyzer -> JSON -> CDN ingest.

## Build
```
cargo build --release
```

## Run analyzer
```
./target/release/ctas7-phd-analyzer .
./target/release/ctas7-phd-analyzer . --json > target/an.json
```

## Post to Statistical Analysis CDN (default :18109/ingest)
```
./target/release/ctas7-phd-analyzer . --json |   ./target/release/post_stats --url http://localhost:18109/ingest   --source ctas7 --tag run
# or
./scripts/send_to_stats.sh . http://localhost:18109/ingest ctas7 $(date -Iseconds)
```

### Payload schema (example)
```json
{
  "kind": "ctas.analysis.features.v1",
  "summary": { "source":"ctas7","tag":"run","ts":"...","root":"...",
    "summary":{"files":10,"loc":4200,"lloc":3500,"avg_cyclo":3.1}},
  "features": [ { "source":"ctas7","tag":"run","ts":"...","path":"crate/src/lib.rs",
    "vector":{"loc":123,"lloc":98,"comments":10,"comment_ratio":0.10,
              "cyclo":5,"cyclo_per_100loc":4.1,"mi":91.2,
              "h_volume":2200.5,"h_difficulty":11.3,"h_effort":24806.0,
              "warns":0} } ]
}
```
