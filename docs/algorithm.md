# The algorithm about BitClock
## What is BitClock?
- Distributed timestamp system
- No ledger or consensus required
- Instant finality

## Problem
In distributed systems:
- Global time does not exist
- It is hard to prove the order

-> **Consensus** is required to determine the order

However:
1. It takes quite some time to finalize.
2. The order might change

## Well then
- What if reaching a consensus **weren't necessary** to determine the order?
- What if the determined order **doesn't change**?

## This is BitClock.
- **No ledger or consensus required**
- **Instant finality**

## How does it work?
By collecting stamps from each node and creating proofs, we establish a timeline within a decentralized system.

## Stamp
A stamp is a local proof of a timeline issued by each node.Each stamp has a local counter, and the value of that counter determines the timeline in the node.

## Proof
