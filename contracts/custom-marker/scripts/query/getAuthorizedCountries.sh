
provenanced query wasm contract-state smart tp1epgd339ckn8k6zlndxfj7cd8rhavzsd56tqf3j6et5vfr75rhyfslhtcrz \
	'{
    "get_authorized_countries": {}
}' \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq
