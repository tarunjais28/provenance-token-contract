
provenanced tx wasm execute \
    tp15wc3aqpd5gqs9xkn84nra6npy930fptujj7u6rjqflc64nptzmsqzc7p7h \
    '{
    "transfer": {
        "recipient": "tp1m97r57ms8dl7pxn0j4m7w80d6a5qvdp7pns80g",
        "amount": "50"
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
