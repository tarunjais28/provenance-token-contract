
provenanced tx wasm execute \
    tp1k2zvmx90efp6r4qfedfakq8xqpzrhvn0d4w6787hckm6j86ch5jsu4mr4l \
    '{
    "update_frozen_list": {
        "sub": {
            "address": "tp1lz7rw3p48tsztjaqpnqzz7vzwfczrlkcrwkgqy",
            "amount": "100"
        }
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
