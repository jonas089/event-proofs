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