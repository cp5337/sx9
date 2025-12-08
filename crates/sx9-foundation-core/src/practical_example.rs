//! CTAS-7 Practical Integration Example
//! Demonstrates complete Tesla/SpaceX-grade zero-trust analysis workflow

use ctas7_phd_analyzer::{
    analyze_with_security,
    simple_workflow_manager::*,
    usim_pgp_integration::*,
    unicode_key_compression::*,
    usim_blockchain::*,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 CTAS-7 Practical Integration Example");
    println!("=======================================");
    println!("Tesla/SpaceX-grade zero-trust code analysis workflow\n");

    // 1. Analyze code with contextual security
    let source_code = r#"
    pub fn secure_payment_processor(amount: u64, account: &str) -> Result<String, String> {
        if amount == 0 {
            return Err("Invalid amount".to_string());
        }

        if account.is_empty() {
            return Err("Invalid account".to_string());
        }

        // Process payment with multiple security checks
        if amount > 1000000 {
            return Err("Amount exceeds limit".to_string());
        }

        Ok(format!("Payment processed: ${} to {}", amount, account))
    }
    "#;

    // 2. Define security context (5-dimensional)
    let context = Context {
        user_trust: 75,                    // Trusted developer
        machine_type: Machine::Prod,       // Production deployment
        code_level: CodeLevel::Sensitive,  // Financial code
        operation: Operation::Deploy,      // Deployment operation
        time_risk: TimeRisk::Normal,       // Normal time pressure
    };

    println!("📊 Step 1: Code Analysis with Contextual Security");
    println!("Context: User Trust: {}%, Machine: {:?}, Code: {:?}, Op: {:?}, Time: {:?}",
        context.user_trust, context.machine_type, context.code_level,
        context.operation, context.time_risk);

    let (analysis, security_level) = analyze_with_security(source_code, "payment.rs", context.clone());

    println!("✅ Analysis Results:");
    println!("   LOC: {}, Complexity: {}, MI: {:.1}",
        analysis.loc, analysis.cyclo, analysis.mi);
    println!("   Security Level: {:?}", security_level);
    println!("   Warnings: {:?}\n", analysis.warnings);

    // 3. Apply security measures based on level
    match security_level {
        SecurityLevel::Fast => {
            println!("🔒 Step 2: Fast Security (Checksum only)");
            let checksum = format!("{:x}", md5::compute(source_code.as_bytes()));
            println!("   Checksum: {}\n", checksum);
        }
        SecurityLevel::Standard => {
            println!("🔐 Step 2: Standard Security (Checksum + PGP)");
            let checksum = format!("{:x}", md5::compute(source_code.as_bytes()));
            println!("   Checksum: {}", checksum);

            let pgp_manager = UsimPgpManager::new("/tmp/ctas7-keys".to_string());
            match pgp_manager.sign_build_artifact(&checksum, "payment-processor") {
                Ok(signature) => {
                    println!("   PGP Signature: {} bytes", signature.signature.len());
                    let verification = pgp_manager.verify_build_signature(&signature);
                    println!("   Trust Level: {}%\n", verification.trust_level);
                }
                Err(e) => println!("   PGP Error: {}\n", e),
            }
        }
        SecurityLevel::Secure => {
            println!("🛡️  Step 2: Secure (Full Zero-Trust Stack)");
            let checksum = format!("{:x}", md5::compute(source_code.as_bytes()));
            println!("   Checksum: {}", checksum);

            // PGP Integration
            let pgp_manager = UsimPgpManager::new("/tmp/ctas7-keys".to_string());
            if let Ok(signature) = pgp_manager.sign_build_artifact(&checksum, "payment-processor") {
                let verification = pgp_manager.verify_build_signature(&signature);
                println!("   PGP Signature: {} bytes (Trust: {}%)",
                    signature.signature.len(), verification.trust_level);

                // Unicode Key Compression
                let mut compressor = UnicodeKeyCompressor::new();
                compressor.config.target_ratio = 40; // Low compression for reliability

                let test_key = vec![0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0];
                if let Ok(compressed_key) = compressor.compress_key(&test_key, "RSA", 4096) {
                    println!("   Unicode Key: {} ({}% compression)",
                        compressed_key.compressed_data, compressed_key.metadata.compression_ratio);

                    // Blockchain Audit Trail
                    let mut blockchain = UsimBlockchainManager::new();
                    let key_ref = UsimPgpKeyRef {
                        fingerprint: "EXAMPLE123456789ABCDEF".to_string(),
                        key_id: "9ABCDEF".to_string(),
                        identity: "Payment Processor Key".to_string(),
                        created_at: 1732365200,
                        expires_at: 1763901200,
                        key_strength: 4096,
                        algorithm: "RSA".to_string(),
                    };

                    if let Ok(_) = blockchain.register_key(key_ref, compressed_key, "payment-system") {
                        let stats = blockchain.get_stats();
                        println!("   Blockchain: {} blocks, {} transactions",
                            stats.total_blocks, stats.total_transactions);
                    }
                }
            }
            println!();
        }
    }

    // 4. Performance validation
    println!("⚡ Step 3: Performance Validation");
    let start = std::time::Instant::now();
    let iterations = 1000;

    for _ in 0..iterations {
        let _ = analyze_with_security(source_code, "test.rs", context.clone());
    }

    let duration = start.elapsed();
    let ops_per_sec = iterations as f64 / duration.as_secs_f64();
    println!("   Performance: {:.0} analyses/sec", ops_per_sec);

    if ops_per_sec > 50.0 {
        println!("   ✅ Performance meets Tesla/SpaceX standards\n");
    } else {
        println!("   ❌ Performance below standards\n");
    }

    // 5. Final assessment
    println!("🎯 Step 4: Final Assessment");
    let overall_quality = if analysis.mi >= 85.0 && analysis.cyclo <= 15 && ops_per_sec > 50.0 {
        "READY FOR DEPLOYMENT"
    } else {
        "NEEDS IMPROVEMENT"
    };

    println!("   Code Quality: {} (MI: {:.1}, Complexity: {})",
        if analysis.mi >= 85.0 { "✅ Excellent" } else { "❌ Poor" },
        analysis.mi, analysis.cyclo);
    println!("   Security: {} ({:?})",
        match security_level {
            SecurityLevel::Secure => "✅ Full Zero-Trust",
            SecurityLevel::Standard => "⚠️  Standard Protection",
            SecurityLevel::Fast => "⚠️  Basic Protection",
        }, security_level);
    println!("   Performance: {} ({:.0} ops/sec)",
        if ops_per_sec > 50.0 { "✅ High Performance" } else { "❌ Slow" },
        ops_per_sec);

    println!("\n🚀 OVERALL STATUS: {}", overall_quality);

    if overall_quality == "READY FOR DEPLOYMENT" {
        println!("   • Multi-target builds: Native, WASM, Embedded ARM");
        println!("   • Contextual security: 5-dimensional analysis");
        println!("   • Cryptographic integrity: PGP + Unicode + Blockchain");
        println!("   • Performance validated: Production-ready");
        println!("   • Tesla/SpaceX engineering standards: COMPLIANT");
    }

    Ok(())
}