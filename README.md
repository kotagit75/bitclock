<div align="center">
    <img src="assets/icon.svg" width=200 height=200>
    <h1>BitClock</h1>
</div>

BitClock is a ledger-free distributed timestamp system. It allows timestamps to be issued without the need for consensus. Each node generates and shares stamps, which are then compiled into a proof, allowing the order in which the data was generated to be verified.

[![License](https://img.shields.io/badge/license-MIT-blue?style=flat)](LICENSE)
[![Open in Visual Studio Code](https://img.shields.io/static/v1?logo=visualstudiocode&label=&message=Open%20in%20Visual%20Studio%20Code&labelColor=2c2c32&color=007acc&logoColor=007acc)](https://open.vscode.dev/kotagit75/bitclock)

> [!NOTE]
> BitClock is currently in active development. The API and features may change without notice.

> [!IMPORTANT]
> BitClock is in the process of migrating from TypeScript to Rust. Development using Rust is underway in this repository.[Proof of Concept Using TypeScript](https://github.com/kotagit75/bitclock-PoC)

## :sparkles: Features
- 🔒 **Instant confirmation** - A proof is finalized once it is validated by the network and cannot be reversed.
- 🏎️ **No consensus required**　- Timestamps can be issued without consensus.
- 🕰️ **Distributed timestamp without ledger** - We can verify timestamp without global ledgers or blockchains.
- ⚡ **Fast timestamp creation** - Once you've collected a certain number of stamps, you can create a proof right away.

## :dart: Use case
### Digital Certificates
With BitClock, you can not only create digital certificates but also prove their order.

## :building_construction: How it works?
1. A client creates a Proof request
2. The request is broadcast to the network
3. Nodes generate Stamps for the Proof
4. Stamps are collected and aggregated
5. Once a threshold is reached:
6. The Proof is finalized
7. It is broadcast to the network

## :clock4: Time sequence
BitClock does not have a global clock.
Instead, each node maintains a counter that serves as its local time, and the timeline is determined by observing changes in these counters. This timeline determination is performed by `compare_time`.

## :books: Documents
- About the algorithm - [algorithm](docs/algorithm.md)

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

# add peer
$ curl -X POST -H "Content-Type: application/json" -d '{"AddPeer":"[peer IP]"}' http://localhost:8080/

# create proof
$ curl -X POST -H "Content-Type: application/json" -d '{"Proof":"[Some data]"}' http://localhost:8080/

# get state(address, secret_key, pool, peers)
$ curl http://localhost:8080/query

# get address
$ curl http://localhost:8080/query/address

# get pool
$ curl http://localhost:8080/query/pool

# get peers
$ curl http://localhost:8080/query/peers

# find proof by secret key
$ curl -X GET -H "Content-Type: application/json" -d '{"der": "[secret_key]"}' http://localhost:8080/query/find

# verify proof
$ curl -X GET -H "Content-Type: application/json" -d '[Proof json]' http://localhost:8080/query/verify

# compare the issuance times of the two proofs
$ curl -X GET -H "Content-Type: application/json" -d '{"sk1":{"der": "[secret_key1]"}, "sk2":{"der": "[secret_key2]"}}' http://localhost:8080/query/compare

# display help
$ cargo run -- -h
Usage: bitclock [OPTIONS]

Options:
  -l, --level <LEVEL>        [default: INFO]
  -h, --help                 Print help
```

> [!CAUTION]
> Never make the `node` directory or any files within it publicly accessible. Doing so could result in the leakage of your private key.

## :jigsaw: APIs
Users can control BitClock via an HTTP server.

| implemented | method | endpoint | feature |
| ---- | ---- | ---- | ---- |
| <ul><li> [x] </ul> | `POST` | / | execute command |
| <ul><li> [x] </ul> | `GET` | /status | get status |
| <ul><li> [x] </ul> | `GET` | /query | get state |
| <ul><li> [x] </ul> | `GET` | /query/address | get address |
| <ul><li> [x] </ul> | `GET` | /query/pool | get proof pool |
| <ul><li> [x] </ul> | `GET` | /query/peers | get peers |
| <ul><li> [x] </ul> | `GET` | /query/compare | compare the issuance times of the two proofs |

### Commands that can be executed at the `/` endpoint
- Add a peer - Post a request with `{"AddPeer": "peerIP"}` in the body
- Create proof - Post a request with `{"Proof": "some data"}` in the body

## :ticket: License
[BitClock is under the MIT License.](LICENSE)
