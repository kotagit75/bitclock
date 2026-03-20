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
$ curl http://localhost:8080/address

# get pool
$ curl http://localhost:8080/pool

# get peers
$ curl http://localhost:8080/peers

# add peer
$ curl -X POST -H "Content-Type: application/json" -d '{"ip":"[peerIP]"}' http://localhost:8080/add-peer

# proof
$ curl -X POST -H "Content-Type: application/json" -d '{"data":"Some data"}' http://localhost:8080/proof
```

## :mag: Learn more
You can find more details on the wiki.
- [Home](https://github.com/kotagit75/bitclock/wiki/)
- [Stamp](https://github.com/kotagit75/bitclock/wiki/Stamp)
- [Proof](https://github.com/kotagit75/bitclock/wiki/Proof)
- [API](https://github.com/kotagit75/bitclock/wiki/API)

## :ticket: License
[BitClock is under the MIT License.](LICENSE)