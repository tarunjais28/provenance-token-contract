provenanced tx wasm instantiate 270 \
	'{
    "name": "Bash Shell",
    "symbol": "BASH",
    "decimals": 6,
    "initial_balances": [
        {
            "address": "tp1lz7rw3p48tsztjaqpnqzz7vzwfczrlkcrwkgqy",
            "amount": "500"
        },
        {
            "address": "tp1m97r57ms8dl7pxn0j4m7w80d6a5qvdp7pns80g",
            "amount": "250"
        }
    ],
    "frozen_balances": [
        {
            "address": "tp1lz7rw3p48tsztjaqpnqzz7vzwfczrlkcrwkgqy",
            "amount": "400"
        }
    ],
    "mint": {
        "minter": "tp1c7apkgl4l8pw72nsh6uvvr7d6fy3c98hyg0sct",
        "cap": "2000"
    },
    "bal_cap": "1000"
}' \
    --admin "$feebucket" \
    --label token-contract \
    --from $feebucket \
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
