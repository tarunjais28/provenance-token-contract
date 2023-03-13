provenanced tx wasm execute \
    tp1pdmrrdr2w9xf0v9qndshg7gtwc6y3qeag3lvr65f9gcpjel62nysrmqpc7 \
    '{
    "destroy": {
        "denom": "SampleCoin"
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
