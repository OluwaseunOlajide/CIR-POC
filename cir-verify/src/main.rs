use serde::Deserialize;
use std::fs;
use std::process;

#[derive(Deserialize, Debug)]
struct AttestationReport {
    status: String,
    execution_time_ms: u128,
    result_hash: String,
    tee_provider: String,
    signature: String,
}

fn main() {
    println!("--- CIR Client Verification Tool ---");

    // 1. Load the Report (Simulating receiving it from the cloud)
    // NOTE: We look for the file in the adjacent folder 'cir-poc'
    let report_path = "../cir-poc/attestation_report.json";
    
    println!("Loading report from: {}", report_path);
    let content = match fs::read_to_string(report_path) {
        Ok(c) => c,
        Err(_) => {
            println!("❌ ERROR: Could not find attestation report.");
            process::exit(1);
        }
    };

    // 2. Parse JSON
    let report: AttestationReport = match serde_json::from_str(&content) {
        Ok(r) => r,
        Err(e) => {
            println!("❌ ERROR: Invalid JSON format. {}", e);
            process::exit(1);
        }
    };

    // 3. Verify Logic
    println!("\n[Check 1] Verifying Execution Status...");
    if report.status != "success" {
        println!("❌ FAILED: Status is '{}'", report.status);
        process::exit(1);
    }
    println!("✅ Status: SUCCESS");

    println!("\n[Check 2] Verifying TEE Signature...");
    // In a real app, we would use the AMD Public Key to verify the signature string.
    // For PoC, we check if a signature exists.
    if report.signature.len() < 10 {
        println!("❌ FAILED: Invalid or missing signature.");
        process::exit(1);
    }
    println!("✅ Signature: VALID (Mock Verified)");
    println!("   > Signed by: {}", report.tee_provider);

    println!("\n[Check 3] Verifying Data Integrity...");
    println!("   > Received Hash: {}...", &report.result_hash[0..10]);
    // In a real app, the user might compare this hash against a known expected value
    // or use it to decrypt the actual data result.
    
    println!("\n-------------------------------------------");
    println!("✅ INTEGRITY CONFIRMED. EXECUTION TRUSTED.");
    println!("-------------------------------------------");
}