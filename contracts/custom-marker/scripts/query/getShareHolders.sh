
provenanced query wasm contract-state smart tp1pdmrrdr2w9xf0v9qndshg7gtwc6y3qeag3lvr65f9gcpjel62nysrmqpc7 \
	'{
    "get_share_holders": {
        "denom": "SampleCoin"
    }
}' \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq