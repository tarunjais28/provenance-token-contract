
provenanced tx wasm execute \
    tp1k2zvmx90efp6r4qfedfakq8xqpzrhvn0d4w6787hckm6j86ch5jsu4mr4l \
    '{
    "update_balances": [
        "tp1c7apkgl4l8pw72nsh6uvvr7d6fy3c98hyg0sct",
        {
            "add": {
                "bal_cap": "1000",
                "frozen_bal": "1000"
            }
        }
    ]
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
