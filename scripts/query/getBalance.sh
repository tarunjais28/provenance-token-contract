echo "Balance for account: tp1f8jfpg8tmewcsu2jjwwkhnzun6d30fzjrghvqs"
provenanced query wasm contract-state smart tp1p8tq94s0psqg4ycq43h76rfuwf44rxsuqlxn46gvr4646angvwms59rrp8 \
	'{
    "balance": {
        "address": "tp1f8jfpg8tmewcsu2jjwwkhnzun6d30fzjrghvqs"
    }
}' \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq

echo
echo "Balance for account: $minter"
provenanced query wasm contract-state smart tp1p8tq94s0psqg4ycq43h76rfuwf44rxsuqlxn46gvr4646angvwms59rrp8 \
	'{
    "balance": {
        "address": "tp1c7apkgl4l8pw72nsh6uvvr7d6fy3c98hyg0sct"
    }
}' \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq

echo
echo "Balance for account: $validator"
provenanced query wasm contract-state smart tp1p8tq94s0psqg4ycq43h76rfuwf44rxsuqlxn46gvr4646angvwms59rrp8 \
	'{
    "balance": {
        "address": "tp1m97r57ms8dl7pxn0j4m7w80d6a5qvdp7pns80g"
    }
}' \
    --testnet \
	--output json \
	--node=https://rpc.test.provenance.io:443 | jq