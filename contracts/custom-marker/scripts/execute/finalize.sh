
provenanced tx wasm execute \
    tp1k2zvmx90efp6r4qfedfakq8xqpzrhvn0d4w6787hckm6j86ch5jsu4mr4l \
    '{
    "finalize": {
        "denom": "MCutomMarker"
    }
}' \
    --from $minter \
    --keyring-backend test \
    --home $prov_path \
    --chain-id pio-testnet-1 \
    --gas 4000000 \
    --gas-prices 26905nhash \
    --broadcast-mode block \
    --yes \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq
