# Tutorial
## 1. Installation
Run the following command to install the software:
```bash
# Clone the repository (or Download ZIP)
git clone https://github.com/kotagit75/bitclock.git

# Navigate to the project directory
cd bitclock

# build
cargo build --release
```
That completes the installation.

## 2. Get Started with BitClock
First, let’s launch BitClock.
```bash
./target/release/bitclock
```
If you see a log like the one below, it has launched successfully.
```
2026-03-26T08:04:17.256Z INFO  [bitclock::boot] BitClock is booting up
2026-03-26T08:04:17.757Z INFO  [bitclock::boot] SystemStatus { status: Running, memory_usage_bytes: Some( ... ) }
2026-03-26T08:04:17.757Z INFO  [bitclock::core::node] Loading the private key from a file: node/key
2026-03-26T08:04:17.774Z INFO  [bitclock::adapter::api] API server is running on http://localhost:8080
2026-03-26T08:04:17.774Z INFO  [bitclock::adapter::p2p] P2P server is running on http://localhost:62697
```
## 3. Pool Check
ProofPool contains approved proofs.You can check the proof in ProofPool using the following command:
```bash
./target/release/cli pool
```
Initially, ProofPool will show that there is nothing there.
```
{"pool":[]}
```
