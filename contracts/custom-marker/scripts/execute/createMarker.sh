
provenanced tx wasm execute \
    tp15wc3aqpd5gqs9xkn84nra6npy930fptujj7u6rjqflc64nptzmsqzc7p7h \
    '{
    "create": {
        "supply": "100000",
        "denom": "TarunCoin",
        "bal_cap": "1000",
        "frozen_bal": "500",
        "country_code": 91
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
