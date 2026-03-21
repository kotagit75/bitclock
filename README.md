<div align="center">
    <img src="assets/icon.svg" width=200 height=200>
    <h1>BitClock</h1>
</div>

BitClock is a ledger-free distributed timestamp system. You can issue timestamps without relying on centralized servers or large-scale blockchains. By each node generate and share stamps and compile them as proof, we can verify the order in which the data was generated.

[![License](http://img.shields.io/badge/license-MIT-blue.svg?style=flat)](LICENSE)

> [!NOTE]
> BitClock is currently in active development. The API and features may change without notice.

> [!IMPORTANT]
> BitClock is in the process of migrating from TypeScript to Rust. Development using Rust is underway in this repository.[Proof of Concept Using TypeScript](https://github.com/kotagit75/bitclock-PoC)

## :sparkles: Features
- 🕰️ Distributed timestamp without ledger - We can verify timestamp without global ledgers or blockchains.
- ⚡ Fast timestamp creation - Once you've collected a certain number of stamps, you can create a proof right away.
- 🔒 Instant confirmation - A proof is finalized once it is validated by the network and cannot be reversed.

## :rocket: Getting Started
### Installation
```bash
# Clone the repository (or Download ZIP)
$ git clone https://github.com/kotagit75/bitclock.git

# Navigate to the project directory
$ cd bitclock
```

### Usage
```bash
# run
$ cargo run

# get status
$ curl http://localhost:8080/status

# get address
$ curl http://localhost:8080/query/address

# get pool
$ curl http://localhost:8080/query/pool

# get peers
$ curl http://localhost:8080/query/peers

# add peer
$ curl -X POST -H "Content-Type: application/json" -d '{"AddPeer":"peerIP"}' http://localhost:8080/

# proof
$ curl -X POST -H "Content-Type: application/json" -d '{"Proof":"Some data"}' http://localhost:8080/
```

> [!CAUTION]
> Never make the `node` directory or any files within it publicly accessible. Doing so could result in the leakage of your private key.

## :ticket: License
[BitClock is under the MIT License.](LICENSE)