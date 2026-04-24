use solana_client::nonblocking::rpc_client::RpcClient;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;
use log::{info, warn, error};

// --- Configuration ---
const TARGET_RPC_URL: &str = "";
const CONCURRENT_WORKERS: usize = 50;
const TEST_DURATION_SECS: u64 = 60;

// --- Internal Metrics State ---
#[derive(Default)]
struct AuditMetrics {
    total_requests: u64,
    successful_requests: u64,
    failed_requests: u64,
    cumulative_latency_ms: u64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();

    info!("Initializing AuraStream AuditCore v0.1.0");
    info!("Targeting RPC: {}", TARGET_RPC_URL);
    info!("Spawning {} concurrent worker threads...", CONCURRENT_WORKERS);

    let client = Arc::new(RpcClient::new(TARGET_RPC_URL.to_string()));
    let metrics = Arc::new(Mutex::new(AuditMetrics::default()));
    let start_time = Instant::now();

    let mut handles = vec![];

    // Spawn concurrent stress-test workers
    for worker_id in 0..CONCURRENT_WORKERS {
        let client_clone = Arc::clone(&client);
        let metrics_clone = Arc::clone(&metrics);

        let handle = tokio::spawn(async move {
            loop {
                // Check if test duration exceeded
                if start_time.elapsed().as_secs() > TEST_DURATION_SECS {
                    break;
                }

                let req_start = Instant::now();
                // Simulating a standard infrastructure health-check query
                let result = client_clone.get_latest_blockhash().await;
                let latency = req_start.elapsed().as_millis() as u64;

                let mut m = metrics_clone.lock().await;
                m.total_requests += 1;
                m.cumulative_latency_ms += latency;

                match result {
                    Ok(_) => {
                        m.successful_requests += 1;
                    }
                    Err(e) => {
                        m.failed_requests += 1;
                        warn!("Worker {} encountered RPC error: {}", worker_id, e);
                    }
                }

                // Prevent immediate rate-limit ban during testing
                tokio::time::sleep(Duration::from_millis(50)).await;
            }
        });

        handles.push(handle);
    }

    // Await all worker threads
    for handle in handles {
        let _ = handle.await;
    }

    // Compile and output final audit report
    let final_metrics = metrics.lock().await;
    info!("--- AuraStream Infrastructure Audit Report ---");
    info!("Target Node       : {}", TARGET_RPC_URL);
    info!("Total Requests    : {}", final_metrics.total_requests);
    info!("Success Rate      : {:.2}%", 
        (final_metrics.successful_requests as f64 / final_metrics.total_requests as f64) * 100.0);
    info!("Failure Rate      : {:.2}%", 
        (final_metrics.failed_requests as f64 / final_metrics.total_requests as f64) * 100.0);
    
    if final_metrics.successful_requests > 0 {
        let avg_latency = final_metrics.cumulative_latency_ms / final_metrics.successful_requests;
        info!("Avg Latency       : {} ms", avg_latency);
    } else {
        error!("CRITICAL: All requests failed. Node is unreachable or blocking traffic.");
    }
    info!("----------------------------------------------");

    Ok(())
}
