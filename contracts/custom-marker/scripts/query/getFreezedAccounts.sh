
provenanced query wasm contract-state smart tp1k2zvmx90efp6r4qfedfakq8xqpzrhvn0d4w6787hckm6j86ch5jsu4mr4l \
	'{
    "get_freezed_accounts": {}
}' \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq
