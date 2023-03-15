
provenanced tx wasm execute \
    tp1epgd339ckn8k6zlndxfj7cd8rhavzsd56tqf3j6et5vfr75rhyfslhtcrz \
    '{
    "transfer": {
        "amount": "300",
        "denom": "RCustomMarker",
        "country_code": 1,
        "to": "tp1lz7rw3p48tsztjaqpnqzz7vzwfczrlkcrwkgqy",
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
    --gas-prices 1905nhash \
    --broadcast-mode block \
    --yes \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq
