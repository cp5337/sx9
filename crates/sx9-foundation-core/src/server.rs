use crate::{Bridge, Platform};
use serde_json;
use warp::Filter;

pub async fn start_server() -> anyhow::Result<()> {
    println!("🌐 Starting CTAS-7 XSD Mux Bridge HTTP Server...");

    let bridge = Bridge::new(Platform::Native);

    // CORS headers
    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["content-type"])
        .allow_methods(vec!["GET", "POST", "OPTIONS"]);

    // Health check endpoint
    let health = warp::path("health")
        .and(warp::get())
        .map(|| {
            warp::reply::json(&serde_json::json!({
                "status": "healthy",
                "service": "CTAS-7 XSD Mux Bridge",
                "version": "7.0.0"
            }))
        });

    // Compress endpoint
    let compress = warp::path("compress")
        .and(warp::post())
        .and(warp::body::json())
        .map(move |body: serde_json::Value| {
            let bridge = Bridge::new(Platform::Native);

            let operation = body["operation"].as_str().unwrap_or("spin");
            let parameters: Vec<String> = body["parameters"]
                .as_array()
                .unwrap_or(&vec![])
                .iter()
                .map(|v| v.as_str().unwrap_or("").to_string())
                .collect();

            let compressed = bridge.compress_operation(operation, &parameters);
            let (expanded_op, expanded_params) = bridge.expand_operation(&compressed);

            warp::reply::json(&serde_json::json!({
                "original": {
                    "operation": operation,
                    "parameters": parameters
                },
                "compressed": compressed,
                "expanded": {
                    "operation": expanded_op,
                    "parameters": expanded_params
                },
                "platform": "native"
            }))
        });

    // Create operation endpoint
    let create = warp::path("create")
        .and(warp::post())
        .and(warp::body::json())
        .map(move |body: serde_json::Value| {
            let bridge = Bridge::new(Platform::Native);

            let operation = body["operation"].as_str().unwrap_or("spin");
            let parameters: Vec<String> = body["parameters"]
                .as_array()
                .unwrap_or(&vec![])
                .iter()
                .map(|v| v.as_str().unwrap_or("").to_string())
                .collect();

            let op = bridge.create_operation(operation, &parameters);
            let xml = bridge.to_xml(&op);
            let route = bridge.route_to_neural_mux(&op);

            warp::reply::json(&serde_json::json!({
                "operation": op,
                "xml": xml,
                "neural_mux_route": route
            }))
        });

    // Demo endpoint
    let demo = warp::path("demo")
        .and(warp::get())
        .map(|| {
            warp::reply::html(r#"
<!DOCTYPE html>
<html>
<head>
    <title>CTAS-7 XSD Mux Bridge Demo</title>
    <style>
        body { font-family: Arial, sans-serif; max-width: 800px; margin: 50px auto; padding: 20px; }
        .container { background: #f5f5f5; padding: 20px; border-radius: 8px; margin: 20px 0; }
        button { background: #007cba; color: white; padding: 10px 20px; border: none; border-radius: 4px; cursor: pointer; }
        button:hover { background: #005a8b; }
        .result { background: #e8f5e8; padding: 15px; border-radius: 4px; margin: 10px 0; }
        .unicode { font-size: 24px; color: #2196F3; }
    </style>
</head>
<body>
    <h1>🚀 CTAS-7 XSD Mux Bridge</h1>
    <p><strong>Multi-platform XSD + Unicode + Neural Mux Bridge</strong></p>

    <div class="container">
        <h3>🔄 Unicode Compression Demo</h3>
        <p>Operation: <code>spin</code> → <span class="unicode">🔄</span> (Unicode compression)</p>
        <button onclick="testCompression()">Test Unicode Compression</button>
        <div id="compression-result"></div>
    </div>

    <div class="container">
        <h3>🛠️ Operation Creation Demo</h3>
        <button onclick="testOperation()">Create XSD Operation</button>
        <div id="operation-result"></div>
    </div>

    <script>
        async function testCompression() {
            const response = await fetch('/compress', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({
                    operation: 'spin',
                    parameters: ['webapp', 'secure']
                })
            });
            const result = await response.json();
            document.getElementById('compression-result').innerHTML = `
                <div class="result">
                    <strong>Original:</strong> ${result.original.operation} [${result.original.parameters.join(', ')}]<br>
                    <strong>Compressed:</strong> <span class="unicode">${result.compressed}</span><br>
                    <strong>Expanded:</strong> ${result.expanded.operation} [${result.expanded.parameters.join(', ')}]
                </div>
            `;
        }

        async function testOperation() {
            const response = await fetch('/create', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({
                    operation: 'spin',
                    parameters: ['container', 'secure']
                })
            });
            const result = await response.json();
            document.getElementById('operation-result').innerHTML = `
                <div class="result">
                    <strong>Neural Mux Route:</strong> ${result.neural_mux_route}<br>
                    <strong>Hash:</strong> ${result.operation.hash}<br>
                    <strong>XML Preview:</strong><br>
                    <pre style="background: white; padding: 10px; border-radius: 4px; overflow-x: auto;">${result.xml}</pre>
                </div>
            `;
        }
    </script>
</body>
</html>
            "#)
        });

    let routes = health
        .or(compress)
        .or(create)
        .or(demo)
        .with(cors);

    println!("✅ Server running on http://localhost:15200 (proxied via https://localhost/mux/)");
    println!("🌐 Demo available at https://localhost/mux/demo");

    warp::serve(routes)
        .run(([0, 0, 0, 0], 15200))
        .await;

    Ok(())
}