provenanced query wasm contract-state smart tp1pdmrrdr2w9xf0v9qndshg7gtwc6y3qeag3lvr65f9gcpjel62nysrmqpc7 \
	'{
    "get_by_address": {
        "address": "tp1y8z4vtls27pauhhqrc4k5mcdx02vhdwa59n66g"
    }
}' \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq
