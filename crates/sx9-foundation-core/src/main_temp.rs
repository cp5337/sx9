use ctas_document_intel::DocumentintelSystem;
use anyhow::Result;
use std::fs;
use walkdir::WalkDir;
use reqwest;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🚀 Starting CTAS Document Manager with LLM Integration...");
    
    let system = DocumentintelSystem::new()?;
    println!("✅ Document Manager initialized with LLM capabilities");
    
    // Find and process markdown files
    println!("📋 Scanning for documents...");
    let mut doc_count = 0;
    let mut processed_docs = Vec::with_capacity(50);
    
    let project_root = std::path::Path::new("../../");
    let mut documents = Vec::with_capacity(50);
    
    for entry in WalkDir::new(project_root)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            let path = e.path();
            path.extension().map_or(false, |ext| ext == "md") &&
            !path.to_string_lossy().contains("target/") &&
            !path.to_string_lossy().contains(".git/") &&
            path.is_file()
        })
    {
        if let Ok(content) = fs::read_to_string(entry.path()) {
            documents.push((entry.path().to_path_buf(), content));
        }
    }
    
    println!("📊 Found {} documents to process", documents.len());
    
    let batch_size = 5;
    for (batch_idx, chunk) in documents.chunks(batch_size).enumerate() {
        println!("🔄 Processing batch {}/{} ({} documents)", 
            batch_idx + 1, (documents.len() + batch_size - 1) / batch_size, chunk.len());
        
        for (path, content) in chunk {
            println!("📄 Processing: {} ({} chars)", path.display(), content.len());
            
            let analysis = system.analyze_document(&content).await?;
            doc_count += 1;
            
            println!("   📊 Analysis: {} words, {} lines, complexity: {:.2}", 
                analysis.word_count, analysis.line_count, analysis.complexity_score);
            println!("   🧠 LLM: Sentiment: {:.2}, Entities: {:?}", 
                analysis.sentiment_score, analysis.key_entities);
            println!("   🚨 Threats: {:?}", analysis.threat_indicators);
            println!("   🔧 Technical: {:?}", analysis.technical_terms);
            println!("   🔗 Contextual Hash: {}", analysis.hash_context.contextual_signature);
            
            let doc_data = json!({
                "input": content,
                "domain": "ctas_documents",
                "analysis": {
                    "sentiment_score": analysis.sentiment_score,
                    "key_entities": analysis.key_entities,
                    "summary": analysis.document_summary,
                    "threat_indicators": analysis.threat_indicators,
                    "technical_terms": analysis.technical_terms,
                    "contextual_analysis": analysis.contextual_analysis,
                    "hash_context": {
                        "content_hash": analysis.hash_context.content_hash,
                        "semantic_hash": analysis.hash_context.semantic_hash,
                        "threat_hash": analysis.hash_context.threat_hash,
                        "technical_hash": analysis.hash_context.technical_hash,
                        "contextual_signature": analysis.hash_context.contextual_signature
                    }
                }
            });
            
            processed_docs.push(doc_data);
        }
        
        println!("   📈 Progress: {} documents processed with LLM analysis...", doc_count);
    }
    
    println!("✅ Processed {} documents with LLM enhancement", doc_count);
    
    // Send documents to Hashing Engine with correct port
    if !processed_docs.is_empty() {
        println!("🔐 Sending enhanced documents to Hashing Engine on port 8080...");
        
        let client = reqwest::Client::new();
        let base_url = format!("http://localhost:8080");
        
        // Register domain
        let domain_registration = json!({
            "input": "ctas_documents_enhanced",
            "domain": "ctas_documents",
            "components": ["content", "semantic", "threat", "technical", "contextual"],
            "weights": [0.3, 0.25, 0.2, 0.15, 0.1],
            "seed": 42
        });
        
        match client.post(&format!("{}/domain", base_url))
            .json(&domain_registration)
            .send()
            .await {
            Ok(response) => {
                if response.status().is_success() {
                    println!("✅ Enhanced domain registered successfully");
                } else {
                    println!("⚠️  Domain registration failed: {}", response.status());
                }
            }
            Err(e) => {
                println!("❌ Failed to register domain: {}", e);
            }
        }
        
        // Send documents for hashing
        for (i, doc) in processed_docs.iter().enumerate() {
            match client.post(&format!("{}/hash", base_url))
                .json(doc)
                .send()
                .await {
                Ok(response) => {
                    if response.status().is_success() {
                        if let Ok(hash_result) = response.json::<serde_json::Value>().await {
                            println!("🔗 Document {}: Enhanced Hash = {}", i + 1, hash_result["hash"]);
                        }
                    } else {
                        println!("⚠️  Hash request failed for document {}: {}", i + 1, response.status());
                    }
                }
                Err(e) => {
                    println!("❌ Failed to send document {} to hashing engine: {}", i + 1, e);
                }
            }
        }
    }
    
    println!("✅ Enhanced Document Manager completed with LLM integration");
    Ok(())
}
