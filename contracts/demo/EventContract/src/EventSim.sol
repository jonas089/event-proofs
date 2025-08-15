// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

event SimpleEvent(uint256 amount, address account);

contract EventSim {
    function fire(uint256 number, address account) public {
        emit SimpleEvent(number, account);
    }
}

// chef$ cast calldata "fire(uint256,address)" 42 0x1234567890abcdef1234567890abcdef12345678
// out: 0x3f389e28000000000000000000000000000000000000000000000000000000000000002a0000000000000000000000001234567890abcdef1234567890abcdef12345678

/*
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
  }'


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

*/

/*
DATA=$(cast calldata "setNumber(uint256)" 10)
echo $DATA
Event located in Block Number: 9


Event topic for this contract: cast keccak "SimpleEvent(uint256,address)" = 0x740677deb12c439a0547bc2722d9141fd4e57c1a92e99934c2802af205870464
*/

/*
cast logs --rpc-url http://127.0.0.1:8545 --from-block 9 --to-block 9 --address 0x663F3ad617193148711d28f5334eE4Ed07016602 "SimpleEvent(uint256 amount, address account)"
*/
