provenanced query wasm contract-state smart tp1k2zvmx90efp6r4qfedfakq8xqpzrhvn0d4w6787hckm6j86ch5jsu4mr4l \
	'{
    "get_by_address": {
        "address": "tp1y8z4vtls27pauhhqrc4k5mcdx02vhdwa59n66g"
    }
}' \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq
