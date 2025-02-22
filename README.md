An implementation of blockchain based off of Bitcoin specs with its corresponding CLI

The networking primitives are WIP

To do: 
- Change println for proper logger (WIP)
- Add collection, validation, and relay of new transactions
- MAX_TX_NEW_BLOCK (For this app we'll keep it set to 2)
- Receive new block -> Stop mining -> Check tx in block, check that all transactions are valid, remove tx from mem pool, recalculate priority and set up candidate block and mine

