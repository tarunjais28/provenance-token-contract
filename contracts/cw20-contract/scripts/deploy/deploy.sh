wasm=/home/tarunjais/Documents/assignment/provenance-token-contract/contracts/cw20-contract/artifacts/token_contract.wasm

provenanced tx wasm store $wasm \
    --instantiate-anyof-addresses "$feebucket" \
    --from "$feebucket" \
    --keyring-backend test \
    --home $prov_path \
    --chain-id pio-testnet-1 \
    --broadcast-mode block \
    --testnet \
    --yes \
    --gas 4000000 \
    --gas-prices 1905nhash \
    --node=https://rpc.test.provenance.io:443 \
	--output json | jq


