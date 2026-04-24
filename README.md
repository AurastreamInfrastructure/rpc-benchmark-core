# AuditCore: Infrastructure Reliability Toolkit

**AuditCore** is an internal benchmarking and reliability toolkit developed by **AuraStream Infrastructure**. 

This project is strictly designed for auditing RPC provider performance, monitoring block production latency, and simulating network congestion scenarios for institutional distributed ledger environments.

### System Architecture
- **Concurrent Load Simulation:** Saturates RPC connection buffers using Tokio asynchronous runtimes to measure degradation under load.
- **Latency Percentile Mapping:** Calculates p50, p90, and p99 response times for critical endpoints (`getRecentBlockhash`, `getMultipleAccounts`).
- **Data Integrity Verification:** Cross-references node responses to ensure state consistency.

### Requirements
- Rust 1.70+
- Linux (Ubuntu 22.04 LTS or Amazon Linux 2023 recommended)
- A dedicated, high-bandwidth VPS

### Compilation
This is a proprietary internal tool. 
```bash
cargo build --release
