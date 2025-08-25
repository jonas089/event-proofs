source .env
cd contracts/demo/EventContract 

# Deploy the contract
forge script script/EventSim.s.sol:EventScript \
  --rpc-url http://127.0.0.1:8545 \
  --sender 0x7761a11bDF55Ca3E177328747a984FBDBc13383a \
  --private-key 0x668ba700dc561c9a159a9886891ea0b441992a9743f9ed72a569d7e7df4ccc53 \
  --broadcast --skip-simulation -vvv

# Send transaction using private key
cast send \
  --rpc-url http://127.0.0.1:8545 \
  --private-key 0x668ba700dc561c9a159a9886891ea0b441992a9743f9ed72a569d7e7df4ccc53 \
  0x253F3a9fb9b3d6237529B420edD449c02523ebe0 \
  "fire(uint256,address)" 42 0x1234567890abcdef1234567890abcdef12345678

# Query the event logs
cast logs \
  --rpc-url http://127.0.0.1:8545 \
  --from-block 0 --to-block 1000 \
  --address 0x253F3a9fb9b3d6237529B420edD449c02523ebe0 \
  "SimpleEvent(uint256 amount, address account)"
