
provenanced tx wasm execute \
    tp1k2zvmx90efp6r4qfedfakq8xqpzrhvn0d4w6787hckm6j86ch5jsu4mr4l \
    '{
    "withdraw": {
        "denom": "MCutomMarker",
        "amount": "800",
        "country_code": 91,
        "balances": {
            "bal_cap": "1000",
            "frozen_bal": "500"
        }
    }
}' \
    --from $tarun \
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
