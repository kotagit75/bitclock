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
ProofPool contains approved proofs. You can check the proof in ProofPool using the following command:
```bash
./target/release/cli pool
```
Initially, ProofPool will show that there is nothing there.
```json
{"pool":[]}
```

## 4. Create Proof
Let's create a proof. You can assign any string to a proof. You can create a proof for the string "Hello world!" using the following command:
```bash
./target/release/cli proof 'Hello world!'
```
If the proof is created successfully, a message will appear.
```
000000000000000...(example)
```
This is the private key specific to this proof. It can be used to identify the proof created.

## 5. Find Proof
Let’s take a look at the proof we created in the previous chapter. If you run `./target/release/cli pool`, you can view the proof you created. However, as the number of proofs increases, it becomes more difficult to locate a specific one. That is why we use the `find` command. The `find` command searches the ProofPool for a proof containing a specific private key (the string output when the proof was created) and displays the results.
```bash
./target/release/cli find '[secret_key]'
```
If the following JSON data is output, the search was successful. The output JSON contains data regarding the proof that was created.
```json
{"data":"Hello world!","stamps":[...],"sk":{"der":"..."},"address":{"der":"..."},"difficulty":3,"time":1774513428556,"sign":[...]}
```

## 5. Verify Proof
If a friend gives you a proof, how can you verify that it is correct? If that proof exists in ProofPool, you’ll know it’s correct. The `verify` command checks the validity of the proof provided as an argument (i.e., whether it is included in the ProofPool).
```bash
./target/release/cli verify '[proof]'
```
For example, let’s verify the proof we just created.
```bash
./target/release/cli verify '{"data":"Hello world!","stamps":[...],"sk":{"der":"..."},"address":{"der":"..."},"difficulty":3,"time":1774513428556,"sign":[...]}'
```
If the proof is correct, `true` will be displayed; if it is incorrect, `false` will be displayed.

## 6.Compare Proof
To compare which of the two proofs was created first, use the `compare` command.
```bash
./target/release/cli compare '[secret_key1]' '[secret_key2]'
```
If the first proof in the arguments was created first, `Less` is displayed; if the first proof in the arguments was created later, `Greater` is displayed.
