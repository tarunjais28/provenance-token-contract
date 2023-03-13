provenanced query wasm contract-state smart tp15wc3aqpd5gqs9xkn84nra6npy930fptujj7u6rjqflc64nptzmsqzc7p7h \
	'{
    "get_by_address": {
        "address": "tp1y8z4vtls27pauhhqrc4k5mcdx02vhdwa59n66g"
    }
}' \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq
