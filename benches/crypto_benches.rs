use criterion::{criterion_group, criterion_main, Criterion};
use riot_api::crypto; // Assuming your crate name is riot_api
use serde_json::json;

fn benchmark_crypto(c: &mut Criterion) {
    // Use a JSON object for encryption/decryption benchmarks
    let data_to_encrypt = json!({ "key": "some test data" }); 

    // Benchmark encryption
    c.bench_function("encrypt_data", |b| b.iter(|| crypto::encrypt_data(&data_to_encrypt).unwrap()));

    // Need encrypted data for decryption benchmark
    let encrypted_data = crypto::encrypt_data(&data_to_encrypt).unwrap();
    c.bench_function("decrypt_data", |b| b.iter(|| crypto::decrypt_data(&encrypted_data).unwrap()));

    // Benchmark signing
    let data_to_sign = json!({ "message": "sign me" });
    c.bench_function("sign_data", |b| b.iter(|| crypto::sign_data(&data_to_sign).unwrap()));

    // Need signature for verification benchmark
    let signature = crypto::sign_data(&data_to_sign).unwrap();
    // Reconstruct the data format expected by verify_signature (if it expects combined data+sig)
    // Note: verify_signature in crypto.rs only takes data and signature separately.
    c.bench_function("verify_signature", |b| b.iter(|| crypto::verify_signature(&data_to_sign, &signature).unwrap()));
}

criterion_group!(benches, benchmark_crypto);
criterion_main!(benches); 