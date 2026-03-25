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
1. It takes quite some time to finalize
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
A stamp is a local proof of a timeline issued by each node.
- Each stamp has a local counter
- the value of that counter determines the timeline in the node
- A stamp is generated with Proof-of-Work
- A stamp is broadcast over the network

## Proof
A proof is a collection of stamps.
- Combines multiple independent stamps
- Does not rely on any single node

## Why proof enables the ordering
Individual stamps are insufficient:
- They are local
- Using stamps does not allow one to assert legitimacy to nodes that joined later

In the case of proofs:
- They aggregate multiple stamps
-> They contain enough information to compare

The proofs are mutually comparable in terms of time order.

## How is the order determined?
For the two proofs, the one with the higher total values of counters of stamps issued by the same node is considered to be later in the timeline.
The comparison process for proofs is defined in the `compare_time` function.

Properties:
- no external input is required
- the same result is always obtained for the same proofs
- All nodes compute the same result
