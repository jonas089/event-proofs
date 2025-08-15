# Simple Counter Contract that emits an event every time it is called

This contract is used for testing merkle proof libraries with the intention to generate ZKPs for events included in the receipts trie against verified ZK light client roots.

## Run a Node locally
Spin up a node using anvil:

```bash
anvil
```

## Depolyment 
Deploy the contract:

```bash
./start.sh
```

Expected output:
```bash
✅  [Success] Hash: 0xbf844715c6e8e84f62ed3dd22cc303af82af7d782f10a5f6432a787a72ae0fbc
Contract Address: 0x8438Ad1C834623CfF278AB6829a248E37C2D7E3f
Block: 3
Paid: 0.00017435934933216 ETH (210296 gas * 0.82911396 gwei)

✅ Sequence #1 on anvil-hardhat | Total Paid: 0.00017435934933216 ETH (210296 gas * avg 0.82911396 gwei)
```

## Call the contract (will emit static event for testing)
```bash
curl -X POST http://127.0.0.1:8545 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc":"2.0",
    "method":"eth_sendTransaction",
    "params":[{
      "from": "0x3C44CdDdB6a900fa2b585dd299e03d12FA4293BC",
      "to": "0x663F3ad617193148711d28f5334eE4Ed07016602",
      "gas": "0x186a0",
      "gasPrice": "0x3b9aca00",
      "value": "0x0",
      "data": "0x3fb5c1cb000000000000000000000000000000000000000000000000000000000000000a"
    }],
    "id":1
  }'
```

## Query for the event using the static Topic

```bash
curl -X POST http://127.0.0.1:8545   -H "Content-Type: application/json"   -d '{
    "jsonrpc":"2.0",
    "method":"eth_call",
    "params":[
      {
        "to": "0x663F3ad617193148711d28f5334eE4Ed07016602",
        "data": "0x8381f58a"
      },
      "latest"
    ],
    "id":1
  }'
```