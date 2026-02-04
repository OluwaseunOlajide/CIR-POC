use rand::Rng; // Or whatever imports you have
use std::time::Instant;
use std::hint::black_box;
use sha2::{Sha256, Digest};
use std::fs::File;
use std::io::Write;
use serde::Serialize;

// 1. Define the Structure of our "Official Report"
#[derive(Serialize)]
struct AttestationReport {
    status: String,
    execution_time_ms: u128,
    result_hash: String,
    tee_provider: String,
    signature: String, // Mock signature
}

fn main() {
    let n = 200; 
    println!("--- CIR PoC: Secure Inference Engine ({}x{}) ---", n, n);

    // 1. Setup Data
    let mut rng = rand::rng(); // Fixed warning
    let a: Vec<i32> = (0..n*n).map(|_| rng.random_range(1..10)).collect(); // Fixed warning
    let b: Vec<i32> = (0..n*n).map(|_| rng.random_range(1..10)).collect();

    // 2. Run Secure Inference
    println!("\n[Step 1] Running Constant-Time Calculation...");
    let start = Instant::now();
    let result = black_box(matmul_constant_time(black_box(&a), black_box(&b), n));
    let duration = start.elapsed().as_millis();
    println!("   > Done in {}ms", duration);

    // 3. Generate Fingerprint
    println!("\n[Step 2] Generating Cryptographic Fingerprint...");
    let hash_bytes = generate_fingerprint(&result);
    let hash_hex = hex::encode(hash_bytes);
    println!("   > Result Hash: {}", hash_hex);

    // 4. Save to JSON (Simulating the TEE responding to the user)
    println!("\n[Step 3] Exporting Attestation Report...");
    
    let report = AttestationReport {
        status: "success".to_string(),
        execution_time_ms: duration,
        result_hash: hash_hex,
        tee_provider: "AMD SEV-SNP (Simulated)".to_string(),
        signature: "MOCK_SIGNATURE_FROM_AMD_PROCESSOR_12345".to_string(),
    };

    let json_output = serde_json::to_string_pretty(&report).unwrap();
    
    let mut file = File::create("attestation_report.json").expect("Unable to create file");
    file.write_all(json_output.as_bytes()).expect("Unable to write data");

    println!("   > SAVED: 'attestation_report.json' created successfully.");
}

fn generate_fingerprint(data: &[i32]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    for num in data {
        hasher.update(num.to_le_bytes());
    }
    hasher.finalize().into()
}

fn matmul_constant_time(a: &[i32], b: &[i32], n: usize) -> Vec<i32> {
    let mut result = vec![0i32; n * n];
    for i in 0..n {
        for k in 0..n {
            let val_a = a[i * n + k];
            for j in 0..n {
                let val_b = b[k * n + j];
                let prod = black_box(val_a).wrapping_mul(black_box(val_b));
                result[i * n + j] = result[i * n + j].wrapping_add(prod);
            }
        }
    }
    result
}