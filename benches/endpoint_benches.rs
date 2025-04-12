use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use actix_web::{test, web, App, dev::Service};
use serde_json::json;
use riot_api::{routes, crypto, models}; // Import necessary modules from your crate
use tokio::runtime::Runtime;

fn benchmark_endpoints(c: &mut Criterion) {
    let runtime = Runtime::new().unwrap();

    // --- Sample Data ---
    let sample_data = json!({
        "name": "John Doe",
        "age": 30,
        "contact": {
            "email": "john@example.com",
            "phone": "123-456-7890"
        }
    });

    let encrypted_data = crypto::encrypt_data(&sample_data).expect("Encryption failed for setup");
    let signature = crypto::sign_data(&sample_data).expect("Signing failed for setup");
    let verify_payload = models::VerifyRequest {
        data: sample_data.clone(),
        signature: signature.clone(),
    };

    // --- Benchmark Group ---
    let mut group = c.benchmark_group("API Endpoints");

    // --- /encrypt Benchmark ---
    group.bench_function(BenchmarkId::new("POST", "/encrypt"), |b| {
        b.to_async(&runtime).iter(|| async {
            let app = test::init_service(App::new()
                .route("/encrypt", web::post().to(routes::encrypt)) // Only include the tested route for isolation
            ).await;
            let req = test::TestRequest::post().uri("/encrypt").set_json(&sample_data).to_request();
            let resp = test::call_service(&app, req).await;
            assert!(resp.status().is_success());
        });
    });

    // --- /decrypt Benchmark ---
    group.bench_function(BenchmarkId::new("POST", "/decrypt"), |b| {
        b.to_async(&runtime).iter(|| async {
            let app = test::init_service(App::new()
                .route("/decrypt", web::post().to(routes::decrypt))
            ).await;
            let req = test::TestRequest::post().uri("/decrypt").set_json(&encrypted_data).to_request();
            let resp = test::call_service(&app, req).await;
            assert!(resp.status().is_success());
        });
    });

    // --- /sign Benchmark ---
    group.bench_function(BenchmarkId::new("POST", "/sign"), |b| {
        b.to_async(&runtime).iter(|| async {
            let app = test::init_service(App::new()
                .route("/sign", web::post().to(routes::sign))
            ).await;
            let req = test::TestRequest::post().uri("/sign").set_json(&sample_data).to_request();
            let resp = test::call_service(&app, req).await;
            assert!(resp.status().is_success());
        });
    });

    // --- /verify Benchmark ---
    group.bench_function(BenchmarkId::new("POST", "/verify"), |b| {
        b.to_async(&runtime).iter(|| async {
            let app = test::init_service(App::new()
                .route("/verify", web::post().to(routes::verify))
            ).await;
            let req = test::TestRequest::post().uri("/verify").set_json(&verify_payload).to_request();
            let resp = test::call_service(&app, req).await;
            assert!(resp.status().is_success()); // Expect 204 No Content
        });
    });

    group.finish();
}

criterion_group!(benches, benchmark_endpoints);
criterion_main!(benches);