
provenanced query wasm contract-state smart tp1epgd339ckn8k6zlndxfj7cd8rhavzsd56tqf3j6et5vfr75rhyfslhtcrz \
	'{
    "get_balances": {
        "address": "tp1c7apkgl4l8pw72nsh6uvvr7d6fy3c98hyg0sct"
      }
}' \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq