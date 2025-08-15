source .env
anvil & sleep 5 &&
cd contracts/demo/EventContract 
forge script script/EventSim.s.sol:EventScript --rpc-url $SEPOLIA_RPC_URL --sender 0x3C44CdDdB6a900fa2b585dd299e03d12FA4293BC --unlocked --broadcast --skip-simulation -vvv & sleep 1 &&
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
      "data": "0x3f389e28000000000000000000000000000000000000000000000000000000000000002a0000000000000000000000001234567890abcdef1234567890abcdef12345678"
    }],
    "id":1
  }' & sleep 1 &&


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
  }' & sleep 1 &&
  

  cast logs --rpc-url http://127.0.0.1:8545 --from-block 0 --to-block 100 --address 0x663F3ad617193148711d28f5334eE4Ed07016602 "SimpleEvent(uint256 amount, address account)"
