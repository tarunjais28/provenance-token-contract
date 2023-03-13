
provenanced tx wasm execute \
    tp15wc3aqpd5gqs9xkn84nra6npy930fptujj7u6rjqflc64nptzmsqzc7p7h \
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
