provenanced tx wasm instantiate 304 \
	'{
    "name": "custom-markerv1.0.0.cm.pb",
    "country_codes": [
        1,
        91
    ]
}' \
    --admin "$feebucket" \
    --label custom-marker \
    --from "$feebucket" \
    --keyring-backend test \
    --home $prov_path \
    --chain-id pio-testnet-1 \
    --gas 4000000 \
    --gas-prices 4500nhash \
    --broadcast-mode block \
    --yes \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq
    