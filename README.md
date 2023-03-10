# Provenance Token Contract

Provenance CosmWasm Token Contract

## Requirements

1. Token should have regular balance data stores that CW-20 has.
2. Token should have a "Frozen Balance" for each account
3. When tokens are transferred, frozen balance should be checked made sure that amount is locked and not able to be transferred
4. Have a balance cap for each token holder (eg. balance cap for each user = 1000, users can only hold up to 1000 tokens.
5. Have to do required checks for minting and transferring to make sure balance cap never goes over the cap for any token holders
6. Lastly, create tests that check that these functions are working properly. 


## Deployment Details

### contract_id
```
270
```

### contract_address
```
tp1p8tq94s0psqg4ycq43h76rfuwf44rxsuqlxn46gvr4646angvwms59rrp8
```
