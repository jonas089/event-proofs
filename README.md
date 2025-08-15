# Prove Smart Contract events in Zero Knowledge

This project is part of our efforts to prove Smart Contract events in Zero knowledge and contains the following:

- Smart Contract that can be triggered to emit events
- Query functionality for logs on an EVM chain
- Merkle Proof code written in Rust to get a proof for a reciept or range of receipts

## Getting Started
### Prerequisites
- foundry
- anvil

### Deploy the contract locally
To spin up anvil and deploy the example contract locally, run:
```bash
./start.sh
```

This will output the contract address, the static topic for our default event and more!

## Verify an Event against a Receipt Trie Root
```bash
cargo test test_verify_receipt -- --nocapture
```

Example output:

```bash
warning: `event-proofs` (lib test) generated 1 warning
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.53s
     Running unittests src/lib.rs (target/debug/deps/event_proofs-0f0bc4d966ee2e0f)

running 1 test
Event decoded: SimpleEvent { amount: 42, address: "0x1234567890AbcdEF1234567890aBcdef12345678" }
test tests::test_verify_receipt ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

boss:event-proofs chef$ 
boss:event-proofs chef$ 
```