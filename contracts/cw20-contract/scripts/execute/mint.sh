
provenanced tx wasm execute \
    tp1p8tq94s0psqg4ycq43h76rfuwf44rxsuqlxn46gvr4646angvwms59rrp8 \
    '{
    "mint": {
        "recipient": "tp1f8jfpg8tmewcsu2jjwwkhnzun6d30fzjrghvqs",
        "amount": "800"
    }
}' \
    --from $minter \
    --keyring-backend test \
    --home $prov_path \
    --chain-id pio-testnet-1 \
    --gas 4000000 \
    --gas-prices 1905nhash \
    --broadcast-mode block \
    --yes \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq
