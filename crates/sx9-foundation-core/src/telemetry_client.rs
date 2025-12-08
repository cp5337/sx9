// ctas7-quality-kit/src/telemetry_client.rs
use reqwest::blocking::Client;
use serde_json::json;
use std::time::Duration;

pub struct Telemetry {
    client: Client,
    targets: Vec<(String, u16, &'static str)>,
}

impl Telemetry {
    pub fn new() -> Self {
        let c = Client::builder().timeout(Duration::from_secs(3))
            .build().unwrap();
        let t = vec![
            ("monitoring".into(), 18108, "/analysis"),
            ("stats".into(), 18109, "/stats"),
            ("dashboard".into(), 18110, "/dashboard"),
            ("gateway".into(), 18200, "/gateway"),
        ];
        Self { client: c, targets: t }
    }

    pub fn send_analysis(&self, service: &str, file: &str, score: f64) {
        let payload = json!({
            "service": service, "file": file, "score": score,
            "ts": chrono::Utc::now().to_rfc3339()
        });
        for (name, port, path) in &self.targets {
            let url = format!("http://localhost:{}{}", port, path);
            match self.client.post(url).json(&payload).send() {
                Ok(r) if r.status().is_success() =>
                    println!("📡 sent to {} ({})", name, port),
                Ok(r) => eprintln!("⚠️  {} {} {}", name, port, r.status()),
                Err(_) => eprintln!("🔌 {} ({}) offline", name, port),
            }
        }
    }
}
